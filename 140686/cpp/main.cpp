#include <cstddef>
#include <cstring>

// Helper to disable optimizations around a value.
static int black_box(int x) {
  volatile int temp = x;
  asm volatile("" ::: "memory");
  return temp;
}

__attribute__((noinline)) int slice_len_from_ptr_end(const char *ptr,
                                                     const char *end) {
  return black_box(end - ptr);
}

static int slice_len(const char *ptr, int len) {
  const char *end = ptr + len;
  return slice_len_from_ptr_end(ptr, end);
}

__attribute__((noinline)) void non_trivial_destructor() {
  asm volatile("" ::: "memory");
}

struct NonTrivialDestructor {
  ~NonTrivialDestructor() { non_trivial_destructor(); }
};

__attribute__((noinline)) int broken(int version) {
  NonTrivialDestructor _has_drop;

  switch (version) {
  case 1:
    return slice_len("aaaaaaa", 8);
  case 2:
    return slice_len("bbbbbbb", 8);
  case 3:
    return slice_len("bbbbbbb", 8);
  default:
    return 42;
  }
}

int main() {
  // Should return 8, unexpectedly returns 0 with old linker.
  return broken(2);
}
