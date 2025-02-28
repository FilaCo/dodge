#ifndef UTIL_VEC_H
#define UTIL_VEC_H

#include "inttypes.h"

// TODO: implement DEFINE_VEC macro

#define DEFINE_VEC(T)                                                          \
  typedef T *vec_##T;                                                          \
  typedef struct {                                                             \
    uint8_t flags;                                                             \
    T buf[];                                                                   \
  } vec_##T##_hdr5;                                                            \
  typedef struct {                                                             \
    uint8_t len;                                                               \
    uint8_t cap;                                                               \
    uint8_t flags;                                                             \
    T buf[];                                                                   \
  } vec_##T##_hdr8;                                                            \
  typedef struct {                                                             \
    uint16_t len;                                                              \
    uint16_t cap;                                                              \
    uint8_t flags;                                                             \
    T buf[];                                                                   \
  } vec_##T##_hdr16;                                                           \
  typedef struct {                                                             \
    uint32_t len;                                                              \
    uint32_t cap;                                                              \
    uint8_t flags;                                                             \
    T buf[];                                                                   \
  } vec_##T##_hdr32;                                                           \
  typedef struct {                                                             \
    uint32_t len;                                                              \
    uint32_t cap;                                                              \
    uint8_t flags;                                                             \
    T buf[];                                                                   \
  } vec_##T##_hdr64;

#endif // UTIL_VEC_H