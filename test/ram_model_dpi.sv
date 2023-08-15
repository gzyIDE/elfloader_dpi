typedef byte unsigned uint8_t;
typedef shortint unsigned uint16_t;
typedef int unsigned uint32_t;
typedef enum logic [2:0] {
  DSIZE_BYTE  = 3'b000,
  DSIZE_HALF  = 3'b001,
  DSIZE_WORD  = 3'b010,
  DSIZE_UBYTE = 3'b100,
  DSIZE_UHALF = 3'b101
} dsize_t;

import "DPI-C" function chandle loadelf_dpi(input string fname);
import "DPI-C" function chandle init_mem();
import "DPI-C" function uint32_t mem_read32(input uint32_t ad, input chandle ptr);
import "DPI-C" function void mem_write32(input uint32_t ad, input uint32_t dt, input chandle ptr);

module ram_model_dpi #(
  parameter string memfile = "main",
  parameter DataW = 32,
  parameter AddrW = 32
)(
  input wire                clk,
  input wire                reset,

  input wire                reA,
  input wire [AddrW-1:0]    addrA,
  output wire [DataW-1:0]   rdtAo,

  input wire                reB,
  input wire                weB,
  input wire [DataW-1:0]    wdtB,
  input wire [AddrW-1:0]    addrB,
  input wire dsize_t        sizeB,
  output wire [DataW-1:0]   rdtBo
);

reg [DataW-1:0]     r_rdtA;
reg [DataW-1:0]     r_rdtB;


//***** RAM Initialization
chandle memory;
initial begin
  if ( memfile == "none" ) begin
    memory = init_mem();
  end else begin
    memory = loadelf_dpi(memfile);
  end
end


//***** RAM Read
logic [DataW-1:0]   c_ram_dtA;
logic [DataW-1:0]   c_ram_dtB;
always_comb begin
  c_ram_dtA = mem_read32(addrA, memory);
  c_ram_dtB = mem_read32(addrB, memory);
end


//***** RAM B Read/Write size
logic [DataW-1:0] c_rdtB;
logic [DataW-1:0] c_wdtB;
always_comb begin
  case ( sizeB )
    DSIZE_BYTE : begin
      case ( addrB[1:0] )
        2'b00 : c_rdtB = {{24{c_ram_dtB[7]}},  c_ram_dtB[7:0]};
        2'b01 : c_rdtB = {{24{c_ram_dtB[15]}}, c_ram_dtB[15:8]};
        2'b10 : c_rdtB = {{24{c_ram_dtB[23]}}, c_ram_dtB[23:16]};
        2'b11 : c_rdtB = {{24{c_ram_dtB[31]}}, c_ram_dtB[31:24]};
      endcase
      case ( addrB[1:0] )
        2'b00 : c_wdtB = {c_ram_dtB[31:8], wdtB[7:0]};
        2'b01 : c_wdtB = {c_ram_dtB[31:16], wdtB[7:0], c_ram_dtB[7:0]};
        2'b10 : c_wdtB = {c_ram_dtB[31:24], wdtB[7:0], c_ram_dtB[15:0]};
        2'b11 : c_wdtB = {wdtB[7:0], c_ram_dtB[23:0]};
      endcase
    end
    DSIZE_HALF : begin
      case ( addrB[1] )
        1'b0 : c_rdtB = {{16{c_ram_dtB[15]}}, c_ram_dtB[15:0]};
        1'b1 : c_rdtB = {{16{c_ram_dtB[31]}}, c_ram_dtB[31:16]};
      endcase
      case ( addrB[1] )
        1'b0 : c_wdtB = {c_ram_dtB[31:16], wdtB[15:0]};
        1'b1 : c_wdtB = {wdtB[15:0], c_ram_dtB[15:0]};
      endcase
    end
    DSIZE_WORD : begin
      c_rdtB = c_ram_dtB;
      c_wdtB = wdtB;
    end
    DSIZE_UBYTE : begin
      case ( addrB[1:0] )
        2'b00 : c_rdtB = {{24{1'b0}}, c_ram_dtB[7:0]};
        2'b01 : c_rdtB = {{24{1'b0}}, c_ram_dtB[15:8]};
        2'b10 : c_rdtB = {{24{1'b0}}, c_ram_dtB[23:16]};
        2'b11 : c_rdtB = {{24{1'b0}}, c_ram_dtB[31:24]};
      endcase
      case ( addrB[1:0] )
        2'b00 : c_wdtB = {c_ram_dtB[31:8], wdtB[7:0]};
        2'b01 : c_wdtB = {c_ram_dtB[31:16], wdtB[7:0], c_ram_dtB[7:0]};
        2'b10 : c_wdtB = {c_ram_dtB[31:24], wdtB[7:0], c_ram_dtB[15:0]};
        2'b11 : c_wdtB = {wdtB[7:0], c_ram_dtB[23:0]};
      endcase
    end
    DSIZE_UHALF : begin
      case ( addrB[1] )
        1'b0 : c_rdtB = {{16{1'b0}}, c_ram_dtB[15:0]};
        1'b1 : c_rdtB = {{16{1'b0}}, c_ram_dtB[31:16]};
      endcase
      case ( addrB[1] )
        1'b0 : c_wdtB = {c_ram_dtB[31:16], wdtB[15:0]};
        1'b1 : c_wdtB = {wdtB[15:0], c_ram_dtB[15:0]};
      endcase
    end
    default : begin
      c_rdtB = {DataW{1'b0}};
      c_wdtB = {DataW{1'b0}};
    end
  endcase
end


//***** Sequential logics
always_ff @(posedge clk) begin
  r_rdtA <= reset ? {DataW{1'b0}} : reA ? c_ram_dtA : r_rdtA;
  r_rdtB <= reset ? {DataW{1'b0}} : reB ? c_rdtB    : r_rdtB;

  if ( weB ) begin
    mem_write32(addrB, c_wdtB, memory);
  end
end

//***** output
assign rdtAo = r_rdtA;
assign rdtBo = r_rdtB;

endmodule
