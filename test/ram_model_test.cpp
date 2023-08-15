#include <stdio.h>
#include <verilated.h>
#include "Vram_model_dpi.h"

int main(int argc, char **argv) {
  Verilated::commandArgs(argc, argv);
  Vram_model_dpi *dut = new Vram_model_dpi();

  dut->clk   = 0;
  dut->reset = 1;
  for (int step = 0; step < 10; step++) {
    dut->eval();
    dut->clk = 1;
    dut->eval();
    dut->clk = 0;
  }
  dut->reset = 0;
  dut->eval();

  // Read only port
  for (int step = 0; step < 100; step++) {
    dut->reA = 1;
    dut->addrA = 0x20000 + step * 4;

    dut->clk = 0;
    dut->eval();
    dut->clk = 1;
    dut->eval();

    printf("rdtA[%d], %08x\n",step ,dut->rdtAo);
  }

  // Read write port
  for (int step = 0; step < 100; step++) {
    dut->weB   = 1;
    dut->addrB = 0x100000 + step * 4;
    dut->sizeB = 2; //word
    dut->wdtB  = step;

    dut->clk = 0;
    dut->eval();
    dut->clk = 1;
    dut->eval();

    dut->weB   = 0;
    dut->reB   = 1;

    dut->clk = 0;
    dut->eval();
    dut->clk = 1;
    dut->eval();

    printf("rdtB[%d], %08x\n",step ,dut->rdtBo);
  }
}
