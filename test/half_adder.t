module half_adder(
    a, b,
    s, c
);
input a, b;
output s, c;

assign s = a #[012012012] b;
assign c = a | b;

endmodule
