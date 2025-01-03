module half_adder_test;
    reg a, b;
    wire s, c;
    
    half_adder_test h (
        .a(a),
        .b(b),
        .s(s),
        .c(c)
    );

    initial begin
        a = 0;
        b = 0;
        #1
        a = 0;
        b = 1;
        #1
        a = 0;
        b = 2;
        #1
        a = 1;
        b = 0;
        #1
        a = 1;
        b = 1;
        #1
        a = 1;
        b = 2;
        #1
        a = 2;
        b = 0;
        #1
        a = 2;
        b = 1;
        #1
        a = 2;
        b = 2;
    end
endmodule