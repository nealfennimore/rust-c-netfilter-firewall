#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <linux/types.h>
#include <linux/netfilter.h>
#include <libnetfilter_queue/libnetfilter_queue.h>

/* returns packet id */
static u_int32_t get_id(struct nfq_data *tb)
{
    int id = 0;
    struct nfqnl_msg_packet_hdr *ph;

    ph = nfq_get_msg_packet_hdr(tb);
    if (ph)
    {
        id = ntohl(ph->packet_id);
    }

    return id;
}

static int cb(struct nfq_q_handle *qh, struct nfgenmsg *nfmsg,
              struct nfq_data *nfa, void *data)
{
    u_int32_t id = get_id(nfa);
    return nfq_set_verdict2(qh, id, NF_ACCEPT, 0, 0, NULL);
}

int main(int argc, char **argv)
{
    struct nfq_handle *h;
    struct nfq_q_handle *qh;
    int fd;
    int rv;
    int queue_num = 0;
    char *buf = calloc(UINT32_MAX, sizeof(char));

    printf("opening library handle\n");
    h = nfq_open();
    if (!h)
    {
        fprintf(stderr, "error during nfq_open()\n");
        exit(1);
    }

    printf("unbinding existing nf_queue handler for AF_INET (if any)\n");
    if (nfq_unbind_pf(h, AF_INET) < 0)
    {
        fprintf(stderr, "error during nfq_unbind_pf()\n");
        exit(1);
    }

    printf("binding nfnetlink_queue as nf_queue handler for AF_INET\n");
    if (nfq_bind_pf(h, AF_INET) < 0)
    {
        fprintf(stderr, "error during nfq_bind_pf()\n");
        exit(1);
    }

    printf("binding this socket to queue '0'\n");
    qh = nfq_create_queue(h, queue_num, &cb, NULL);
    if (!qh)
    {
        fprintf(stderr, "error during nfq_create_queue()\n");
        exit(1);
    }

    printf("setting copy_packet mode\n");
    if (nfq_set_mode(qh, NFQNL_COPY_PACKET, 0xffff) < 0)
    {
        fprintf(stderr, "can't set packet_copy mode\n");
        exit(1);
    }

    fd = nfq_fd(h);

    while ((rv = recv(fd, buf, UINT32_MAX, MSG_DONTWAIT)))
    {
        if(rv < 0){
            continue;
        }
        nfq_handle_packet(h, buf, rv);
    }

    printf("unbinding from queue 0\n");
    nfq_destroy_queue(qh);

#ifdef INSANE
    /* normally, applications SHOULD NOT issue this command, since
     * it detaches other programs/sockets from AF_INET, too ! */
    printf("unbinding from AF_INET\n");
    nfq_unbind_pf(h, AF_INET);
#endif

    printf("closing library handle\n");
    nfq_close(h);
    free(buf);

    exit(0);
}