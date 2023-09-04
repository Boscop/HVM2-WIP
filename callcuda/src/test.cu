#include <stdint.h>

typedef uint8_t u8;
typedef uint32_t u32;
typedef uint64_t u64;

// Pointers = 4-bit tag + 28-bit val
typedef u32 Ptr;

// Nodes are pairs of pointers
typedef struct alignas(8) {
  Ptr ports[2];
} Node;

// Wires are pairs of pointers
typedef struct alignas(8) {
  Ptr lft;
  Ptr rgt;
} Wire;

// An interaction net
typedef struct {
  Ptr   root; // root wire
  u32   blen; // total bag length (redex count)
  Wire* bags; // redex bags (active pairs)
  Node* node; // memory buffer with all nodes
  u32*  gidx; // aux buffer used on scatter fns
  Wire* gmov; // aux buffer used on scatter fns
  u32   pbks; // last blocks count used
  u32   done; // number of completed threads
  u32   rwts; // number of rewrites performed
} Net;

extern "C" __global__ void test(Net net) {
    assert(net.root == 0);
    assert(net.blet == 0);
    assert(net.pbks == 0);
    assert(net.done == 0);
    assert(net.rwts == 0);
}