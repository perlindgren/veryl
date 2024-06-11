module veryl_testcase_Module40;

    typedef enum logic [2-1:0] {
        EnumA_member_a,

        EnumA_member_b,
        EnumA_member_c = 3
    } EnumA;

    EnumA a;

    always_comb a = EnumA_member_a;
    function automatic logic is_a(
        input EnumA e
    ) ;
        return e == EnumA_member_a;
    endfunction
endmodule
//# sourceMappingURL=../map/testcases/sv/40_enum_resolve.sv.map
