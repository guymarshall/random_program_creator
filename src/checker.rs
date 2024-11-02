#[inline(always)]
pub(crate) fn code_is_valid(code: &str) -> bool {
    let contains_main: bool =
        code.contains("int main()") || code.contains("int main(int argc, char **argv)");

    let contains_return_zero: bool = code.contains("return 0;");

    contains_main && contains_return_zero
}
