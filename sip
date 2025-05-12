//link C libs
clink "stdlib.h" "unistd.h" "stdio.h";
//extern functions
extern printf ... /*use ... for variadics*/ = "printf"
extern write fd(i32) /*optional type with (type) system */ text len(i32) = "write" fd text len;write(1,"Hi\n",3);
printf("Hello world");
