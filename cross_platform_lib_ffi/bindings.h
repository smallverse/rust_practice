#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Quaternion {
  float x;
  float y;
  float z;
  float w;
};

extern "C" {

uintptr_t add(uintptr_t left, uintptr_t right);

Quaternion gen_quaternion(float x, float y, float z, float w);

char *gen_quaternion_str(float x, float y, float z, float w);

void gen_quaternion_str_free(char *s);

char *gen_obj_info(const char *name, float age, const char *desc);

void gen_obj_info_free(char *s);

} // extern "C"
