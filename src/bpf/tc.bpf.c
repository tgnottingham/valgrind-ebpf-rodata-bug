#include "vmlinux.h"
#include <bpf/bpf_helpers.h>

#define TC_ACT_OK     0

SEC("tc")
int handle_tc(struct __sk_buff *skb) {
    return TC_ACT_OK;
}

char _license[] SEC("license") = "GPL";
