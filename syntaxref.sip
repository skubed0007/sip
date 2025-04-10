@fun test(i32 age);
fun main(){
    lazypage mem = _PAGE(4M);
    nil (mem) {
        @c {
            nprintf("error allocating lazy page!");
        }
    }
    zeropage zmem = _PAGE(4M);
    //shall cause error as `nil` isnt checked for zmem
    i32 age = _ALLOCMEM(zmem,12);
}
fun test(i32 age){
    //code
}