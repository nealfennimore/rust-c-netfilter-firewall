#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <linux/types.h>
#include <linux/netfilter.h>
#include <libnetfilter_queue/libnetfilter_queue.h>

static int cb(struct nfq_q_handle *qh, struct nfgenmsg *nfmsg,
              struct nfq_data *nfa, void *data)
{
    int id = 0;
    struct nfqnl_msg_packet_hdr *ph;
    ph = nfq_get_msg_packet_hdr(nfa);
    if (ph)
    {
        id = ntohl(ph->packet_id);
    }
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

    h = nfq_open();
    if (!h)
    {
        fprintf(stderr, "error during nfq_open()\n");
        exit(1);
    }

    if (nfq_unbind_pf(h, AF_INET) < 0)
    {
        fprintf(stderr, "error during nfq_unbind_pf()\n");
        exit(1);
    }

    if (nfq_bind_pf(h, AF_INET) < 0)
    {
        fprintf(stderr, "error during nfq_bind_pf()\n");
        exit(1);
    }

    qh = nfq_create_queue(h, queue_num, &cb, NULL);
    if (!qh)
    {
        fprintf(stderr, "error during nfq_create_queue()\n");
        exit(1);
    }

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
    free(buf);
    nfq_destroy_queue(qh);
    nfq_close(h);

    exit(0);
}