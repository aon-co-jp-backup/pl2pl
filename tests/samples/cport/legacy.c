#include <stdio.h>
#include <string.h>
#include <strings.h>
#include <unistd.h>

int main(void) {
    char buf[16];
    bzero(buf, sizeof buf);
    strcpy(buf, "Hello");
    char *p = index(buf, 'l');
    if (strcasecmp(buf, "HELLO") == 0 && p) {
        printf("ok %s\n", p);
    }
    usleep(1000);
    return 0;
}
