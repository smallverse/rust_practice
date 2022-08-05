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

struct ObjInfo {
  const char *name;
  float age;
  const char *desc;
};

extern "C" {

uintptr_t add(uintptr_t left, uintptr_t right);

Quaternion gen_quaternion(float x, float y, float z, float w);

char *gen_quaternion_str(float x, float y, float z, float w);

ObjInfo gen_obj_info(const char *name, float age, const char *desc);

} // extern "C"
