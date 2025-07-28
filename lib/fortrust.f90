module math_mod
    implicit none
contains
    subroutine add(a, b, c) bind(C, name="add")
        use iso_c_binding, only: c_int
        integer(c_int), intent(in) :: a, b
        integer(c_int) :: c
        c = a + b
    end subroutine add
end module math_mod
