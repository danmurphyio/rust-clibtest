#include <stdio.h>

extern void rust_add(int left, int right, int *result);
extern int rust_register_datasource(int *ds[], int length);

extern int rust_set_bit(int *val, int bit);
extern int rust_clear_bit(int *val, int bit);

extern int rust_read_db(char *name);

int var0 = 10;
int var1 = 20;
int var2 = 30;
int var3 = 40;
int var4 = 50;
int var5 = 60;
int var6 = 70;
int var7 = 80;
int var8 = 90;
int var9 = 100;

int *regs[] = {
    &var0,
    &var1,
    &var2,
    &var3,
    &var4,
    &var5,
    &var6,
    &var7,
    &var8,
    &var9,
};

int main() {
    int result = 99;

    rust_add(10, 20, &result);

    printf("result = %d\n", result);

    printf("=================================\n");
    for (int i = 0; i < 10; ++i) {
        printf("[%d] -> %d\n", i, *regs[i]);
    }
    printf("---------------------------------\n");
    rust_register_datasource(regs, 10);
    for (int i = 0; i < 10; ++i) {
        printf("[%d] -> %d\n", i, *regs[i]);
    }
    printf("---------------------------------\n");

    printf("var0 = %d\n", var0);
    printf("var1 = %d\n", var1);
    printf("var2 = %d\n", var2);
    printf("var3 = %d\n", var3);
    printf("var4 = %d\n", var4);
    printf("var5 = %d\n", var5);
    printf("var6 = %d\n", var6);
    printf("var7 = %d\n", var7);
    printf("var8 = %d\n", var8);
    printf("var9 = %d\n", var9);
    printf("=================================\n");

    int bit_test_val = 0;
    printf("result = %d, bit_test_val = %d\n", rust_set_bit(&bit_test_val, 7), bit_test_val);
    printf("result = %d, bit_test_val = %d\n", rust_clear_bit(&bit_test_val, 7), bit_test_val);

    rust_read_db("test.db");

    return 0;
}