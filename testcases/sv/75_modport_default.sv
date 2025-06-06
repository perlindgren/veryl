interface veryl_testcase_Interface75;
    logic a;
    logic b;
    logic c;
    logic d;

    modport master (
        input  a,
        input  b,
        output c,
        output d
    );

    modport slave (
        output a,
        output b,
        input  c,
        input  d
    );

    modport all_input (
        input a,
        input b,
        input c,
        input d
    );

    modport all_output (
        input a,
        input b,
        input c,
        input d
    );

    modport partial_converse (
        input  a,
        output b,
        input  c,
        input  d
    );

    modport partial_input (
        output c,
        input  a,
        input  b,
        input  d
    );

    modport master2 (
        input  a,
        input  b,
        output c,
        output d
    );

    modport partial_same (
        output a,
        input  b,
        output c,
        output d
    );
endinterface
//# sourceMappingURL=../map/testcases/sv/75_modport_default.sv.map
