module test(
    input wire [1:0] a,
    input wire b,
    output wire c
);

assign c = a;

always @(a) begin
    c = a;

    if (a) begin
        c = a;
    end
end

endmodule
