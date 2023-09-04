// #include <stdint.h>
typedef unsigned char      uint8_t;
typedef unsigned short     uint16_t;
typedef unsigned int       uint32_t;
typedef unsigned long long uint64_t;

typedef uint8_t u8;
typedef uint32_t u32;
typedef uint64_t u64;

// Configuration
// -------------

// This code is initially optimized for nVidia RTX 4090
const u32 BLOCK_LOG2    = 8;                                     // log2 of block size
const u32 BLOCK_SIZE    = 1 << BLOCK_LOG2;                       // threads per block
const u32 UNIT_SIZE     = 4;                                     // threads per rewrite unit
const u32 NODE_SIZE     = 1 << 28;                               // max total nodes (2GB addressable)
const u32 BAGS_SIZE     = BLOCK_SIZE * BLOCK_SIZE * BLOCK_SIZE;  // size of global redex bag
const u32 GROUP_SIZE    = BLOCK_SIZE * BLOCK_SIZE;               // size os a group of bags
const u32 GIDX_SIZE     = BAGS_SIZE + GROUP_SIZE + BLOCK_SIZE;   // aux object to hold scatter indices
const u32 GMOV_SIZE     = BAGS_SIZE;                             // aux object to hold scatter indices

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

extern "C" __global__ void test(Net* net) {
    int id = blockIdx.x * blockDim.x + threadIdx.x;
    if (id == 0) {
        return;
    }

    assert(net->root == 0);
    assert(net->blen == 0);
    assert(net->pbks == 0);
    assert(net->done == 0);
    assert(net->rwts == 0);

    assert(net->bags != NULL);
    assert(net->node != NULL);
    assert(net->gidx != NULL);
    assert(net->gmov != NULL);

    assert(net->bags[0].lft == 10);
    assert(net->bags[0].rgt == 20);
    for (u32 i = 1; i < BAGS_SIZE; ++i) {
        assert(net->bags[i].lft == 0);
        assert(net->bags[i].rgt == 0);
    }
    /* for (u32 i = 0; i < NODE_SIZE; ++i) {
        assert(net->node->ports[0] == 0);
        assert(net->node->ports[1] == 0);
    }
    for (u32 i = 0; i < GIDX_SIZE; ++i) {
        assert(net->gidx[i] == 0);
    }
    for (u32 i = 0; i < GMOV_SIZE; ++i) {
        assert(net->gmov[i].lft == 0);
        assert(net->gmov[i].rgt == 0);
    } */
}