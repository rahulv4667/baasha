#include <stdint.h>
#include <stdio.h>
#include <stdbool.h>

void printi8(int8_t a) {
    printf("%hhd", a);
}

void printi16(int16_t a) {
    printf("%hd", a);
}

void printi32(int32_t a) {
    printf("%d", a);
}

void printi64(int64_t a) {
    printf("%ld", a);
}

void printu8(uint8_t a) {
    printf("%hhu", a);
}

void printu16(uint16_t a) {
    printf("%hu", a);
}

void printu32(uint32_t a) {
    printf("%u", a);
}

void printu64(uint64_t a) {
    printf("%lu", a);
}

int8_t scani8() {
    int8_t a;
    scanf("%hhd", &a);
    return a;
}

int16_t scani16() {
    int16_t a;
    scanf("%hd", &a);
    return a;
}

int32_t scani32() {
    int32_t a;
    scanf("%d", &a);
    return a;
}

int64_t scani64() {
    int64_t a;
    scanf("%ld", &a);
    return a;
}

uint8_t scanu8() {
    uint8_t a;
    scanf("%hhu", &a);
    return a;
}

uint16_t scanu16() {
    uint16_t a;
    scanf("%hu", &a);
    return a;
}

uint32_t scanu32() {
    uint32_t a;
    scanf("%u", &a);
    return a;
}

uint64_t scanu64() {
    uint64_t a;
    scanf("%lu", &a);
    return a;
}

void printf32(float a) {
    printf("%f", a);
}

void printf64(double a) {
    printf("%lf", a);
}

float scanf32() {
    float a;
    scanf("%f", &a);
    return a;
}

float scanf64() {
    double a;
    scanf("%lf", &a);
    return a;
}

void printbool(bool a) {
    printf("%s", a?"true":"false");
}

bool scanbool() {
    bool a;
    int32_t b;
    scanf("%d", &b);
    a = b>0?true:false;
    return a;
}

void println() {
    printf("\n");
}