#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct PubQuaternion {
  float x;
  float y;
  float z;
  float w;
};

struct PubObjInfo {
  const char *name;
  float age;
  const char *desc;
};

extern "C" {

uintptr_t add(uintptr_t left, uintptr_t right);

PubQuaternion gen_quaternion(float x, float y, float z, float w);

char *gen_quaternion_str(float x, float y, float z, float w);

void gen_quaternion_str_free(char *s);

char *gen_obj_info_str(const char *name, float age, const char *desc);

void gen_obj_info_str_free(char *s);

PubObjInfo *gen_obj_info(const char *name, float age, const char *desc);

void gen_obj_info_free(PubObjInfo *ptr);

} // extern "C"
