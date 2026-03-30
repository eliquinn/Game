#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

void input(char *str);
int cmd(char *str);
int main();

char opt[64];

typedef enum {
    NONE,
    LOOP,
    EXIT,
} State;

void loop() {
    while(1) {
        if (cmd("loud")) {
            printf("AAAAAA\n");
        } else if (cmd("soft")) {
            printf("aaaa...\n");
        } else if (cmd("quit")  || cmd("q")) {
            return;
        } else {
            printf("no command\n");
        }
        input(opt);
    }
}

int cmd(char *str) {
    if (strcmp(opt, str) == 0) {
        return 1;
    } else {
        return 0;
    }
}

void input(char *str) {
    printf(">");
    fgets(str, 64, stdin);
    str[strcspn(str, "\n")] = '\0';
}

int main() {
    State state = NONE;
    input(opt);
    printf("You said %s", opt);
    loop();
    return 0;
}
