module veryl_testcase_Module51;
    logic _a [0:2-1];
    always_comb _a = '{1, 1};
    logic _b [0:2-1];
    always_comb _b = '{2{1}};
    logic _c [0:2-1];
    always_comb _c = '{default: 1};
endmodule
//# sourceMappingURL=../map/testcases/sv/51_array_literal.sv.map
