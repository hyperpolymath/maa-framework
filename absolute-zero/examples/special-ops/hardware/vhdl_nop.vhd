-- VHDL Hardware Description: Computational No-Operation (CNO) Circuit
-- Demonstrates that even identity operations consume silicon resources
-- File: vhdl_nop.vhd

library IEEE;
use IEEE.STD_LOGIC_1164.ALL;
use IEEE.NUMERIC_STD.ALL;

-- =============================================================================
-- Entity: cno_processor
-- Description: A hardware CNO that propagates input to output while consuming
--              clock cycles, power, and silicon area
-- =============================================================================
entity cno_processor is
    generic (
        DATA_WIDTH : integer := 32;
        PIPELINE_STAGES : integer := 3  -- Number of pipeline stages for CNO
    );
    port (
        clk     : in  std_logic;
        reset   : in  std_logic;
        enable  : in  std_logic;
        data_in : in  std_logic_vector(DATA_WIDTH-1 downto 0);
        valid_in: in  std_logic;
        data_out: out std_logic_vector(DATA_WIDTH-1 downto 0);
        valid_out: out std_logic;
        busy    : out std_logic
    );
end cno_processor;

architecture behavioral of cno_processor is

    -- Pipeline registers for multi-cycle CNO
    type pipe_array is array (0 to PIPELINE_STAGES-1) of
         std_logic_vector(DATA_WIDTH-1 downto 0);
    signal pipeline_data : pipe_array;
    signal pipeline_valid : std_logic_vector(PIPELINE_STAGES-1 downto 0);

    -- State machine for CNO execution
    type state_type is (IDLE, PROCESSING, DONE);
    signal current_state : state_type;

    -- Cycle counter (CNO consumes exactly N cycles)
    signal cycle_count : integer range 0 to PIPELINE_STAGES;

begin

    -- ==========================================================================
    -- GATE-LEVEL IMPLEMENTATION ANALYSIS:
    --
    -- For 32-bit data with 3-stage pipeline:
    -- - Pipeline registers: 3 × 32 D-flip-flops = 96 DFFs
    --   Each DFF = ~6 transistors = 576 transistors just for data path
    -- - Valid bit pipeline: 3 DFFs = 18 transistors
    -- - State machine: 2 DFFs (3 states) = 12 transistors
    -- - Cycle counter: 2-bit counter = 4 DFFs = 24 transistors
    -- - Control logic: ~50-100 gates = 300-600 transistors
    --
    -- TOTAL: ~950-1,250 transistors for hardware CNO
    --
    -- Each clock cycle, ALL these transistors experience switching activity:
    -- - Clock distribution network charges/discharges capacitance
    -- - Setup and hold path transistors toggle
    -- - Combinational logic evaluates (even if result unchanged)
    -- ==========================================================================

    -- Main CNO pipeline process
    process(clk, reset)
    begin
        if reset = '1' then
            -- Reset all pipeline stages
            for i in 0 to PIPELINE_STAGES-1 loop
                pipeline_data(i) <= (others => '0');
                pipeline_valid(i) <= '0';
            end loop;
            current_state <= IDLE;
            cycle_count <= 0;

        elsif rising_edge(clk) then

            case current_state is

                when IDLE =>
                    if enable = '1' and valid_in = '1' then
                        -- Load input into first pipeline stage
                        pipeline_data(0) <= data_in;
                        pipeline_valid(0) <= '1';
                        current_state <= PROCESSING;
                        cycle_count <= 1;
                    end if;

                when PROCESSING =>
                    -- Propagate through pipeline stages
                    -- This is the CNO: data passes unchanged, but consumes cycles
                    for i in 1 to PIPELINE_STAGES-1 loop
                        pipeline_data(i) <= pipeline_data(i-1);
                        pipeline_valid(i) <= pipeline_valid(i-1);
                    end loop;

                    cycle_count <= cycle_count + 1;

                    if cycle_count = PIPELINE_STAGES then
                        current_state <= DONE;
                    end if;

                when DONE =>
                    current_state <= IDLE;

            end case;
        end if;
    end process;

    -- Output assignment
    data_out <= pipeline_data(PIPELINE_STAGES-1);
    valid_out <= '1' when (current_state = DONE) else '0';
    busy <= '1' when (current_state = PROCESSING) else '0';

    -- ==========================================================================
    -- POWER CONSUMPTION ANALYSIS:
    --
    -- Dynamic Power (P_dynamic = α × C × V² × f):
    -- - α (switching activity): 0.3-0.5 per cycle (30-50% of gates toggle)
    -- - C (capacitance): ~1 fF per transistor × 1000 transistors = 1 pF
    -- - V (voltage): 1.0V (modern process)
    -- - f (frequency): 100 MHz
    -- - P_dynamic ≈ 0.4 × 1pF × 1.0² × 100MHz = 40 µW
    --
    -- Static Power (leakage):
    -- - Modern 28nm process: ~1 nA per transistor leakage
    -- - 1000 transistors × 1nA × 1.0V = 1 µW
    --
    -- TOTAL POWER: ~41 µW per CNO operation
    --
    -- Over 1 billion CNOs per second: 41 mW wasted on identity operations
    -- Over 1 year continuous: 41mW × 8760 hours = 359 kJ = 0.1 kWh
    -- ==========================================================================

    -- ==========================================================================
    -- PROPAGATION DELAY ANALYSIS:
    --
    -- Critical path for CNO operation:
    -- 1. Clock-to-Q delay of input register: ~50 ps (28nm process)
    -- 2. State machine logic: ~100 ps (2-3 gate delays)
    -- 3. Multiplexer for pipeline stage: ~80 ps
    -- 4. Setup time of output register: ~40 ps
    --
    -- Total combinational delay: ~270 ps
    -- Maximum frequency: 1 / 270ps ≈ 3.7 GHz
    --
    -- Practical design with margin: 1 GHz @ 1.0V
    -- With voltage scaling (0.8V): 500 MHz
    --
    -- Even though CNO does nothing computationally, the physical delay through
    -- silicon is real and limits system performance.
    -- ==========================================================================

end behavioral;


-- =============================================================================
-- Alternative: Combinational CNO (appears to be zero-delay, but isn't)
-- =============================================================================
entity combinational_cno is
    generic (
        DATA_WIDTH : integer := 32
    );
    port (
        data_in : in  std_logic_vector(DATA_WIDTH-1 downto 0);
        data_out: out std_logic_vector(DATA_WIDTH-1 downto 0)
    );
end combinational_cno;

architecture rtl of combinational_cno is
begin

    -- The "wire" CNO - appears to be just a connection
    -- But at gate level, this still involves:
    -- - Buffer insertion for drive strength
    -- - Wire capacitance charging/discharging
    -- - Signal integrity management

    data_out <= data_in;

    -- ==========================================================================
    -- PHYSICAL IMPLEMENTATION:
    --
    -- Even this "simple wire" translates to:
    -- 1. Buffer tree for fanout (if data_out drives multiple loads)
    --    - Each buffer: 4-6 transistors
    --    - For fanout of 10: 2-stage buffer tree = 8 buffers = 48 transistors
    --
    -- 2. Wire parasitics:
    --    - 32 wires × 100 µm average length = 3.2 mm total wire
    --    - Wire capacitance: ~0.2 fF/µm × 3200 µm = 640 fF
    --    - Wire resistance: ~0.1 Ω/µm × 100 µm = 10 Ω per wire
    --
    -- 3. RC delay:
    --    - τ = RC = 10Ω × 20fF = 200 fs per wire segment
    --    - Multi-segment: ~1-2 ps total
    --
    -- 4. Dynamic power per transition:
    --    - P = C × V² × f = 640fF × 1.0² × 1GHz = 640 µW (when switching)
    --
    -- The "free" wire is actually expensive in power and delay!
    -- ==========================================================================

end rtl;


-- =============================================================================
-- Testbench for CNO Processor
-- =============================================================================
entity tb_cno_processor is
end tb_cno_processor;

architecture behavioral of tb_cno_processor is

    constant CLK_PERIOD : time := 10 ns;  -- 100 MHz
    constant DATA_WIDTH : integer := 32;

    signal clk : std_logic := '0';
    signal reset : std_logic := '1';
    signal enable : std_logic := '0';
    signal data_in : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal valid_in : std_logic := '0';
    signal data_out : std_logic_vector(DATA_WIDTH-1 downto 0);
    signal valid_out : std_logic;
    signal busy : std_logic;

    component cno_processor is
        generic (
            DATA_WIDTH : integer := 32;
            PIPELINE_STAGES : integer := 3
        );
        port (
            clk     : in  std_logic;
            reset   : in  std_logic;
            enable  : in  std_logic;
            data_in : in  std_logic_vector(DATA_WIDTH-1 downto 0);
            valid_in: in  std_logic;
            data_out: out std_logic_vector(DATA_WIDTH-1 downto 0);
            valid_out: out std_logic;
            busy    : out std_logic
        );
    end component;

begin

    -- Instantiate CNO processor
    uut: cno_processor
        generic map (
            DATA_WIDTH => DATA_WIDTH,
            PIPELINE_STAGES => 3
        )
        port map (
            clk => clk,
            reset => reset,
            enable => enable,
            data_in => data_in,
            valid_in => valid_in,
            data_out => data_out,
            valid_out => valid_out,
            busy => busy
        );

    -- Clock generation
    clk <= not clk after CLK_PERIOD/2;

    -- Stimulus process
    process
    begin
        wait for CLK_PERIOD * 2;
        reset <= '0';
        wait for CLK_PERIOD;

        -- Test CNO operation: input value should pass through unchanged
        -- but consume 3 clock cycles
        data_in <= x"DEADBEEF";
        valid_in <= '1';
        enable <= '1';
        wait for CLK_PERIOD;

        enable <= '0';
        valid_in <= '0';

        -- Wait for CNO to complete
        wait until valid_out = '1';

        -- Verify: output = input (identity operation)
        assert data_out = x"DEADBEEF"
            report "CNO failed: output != input"
            severity error;

        -- But it consumed 3 clock cycles = 30 ns @ 100MHz
        -- That's 30 ns of wasted computation time
        -- With power consumption of ~41 µW × 30ns = 1.23 pJ energy wasted

        wait for CLK_PERIOD * 5;
        std.env.stop;
    end process;

end behavioral;
