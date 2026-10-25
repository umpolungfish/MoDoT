package FdeTypes;
    typedef shortreal mass_t;
    typedef struct packed { mass_t T; mass_t F; mass_t B; mass_t N; } fde_ledger_t;
endpackage

module fde_inverter (
    input  FdeTypes::fde_ledger_t m_in,
    output FdeTypes::fde_ledger_t m_out
);
    always_comb begin
        m_out.T = m_in.F;  m_out.F = m_in.T;
        m_out.B = m_in.B;  m_out.N = m_in.N;
    end
endmodule

module fde_breathing_gate (
    input  logic                  clk,
    input  FdeTypes::mass_t       gamma_field,
    input  FdeTypes::fde_ledger_t m_in,
    output FdeTypes::fde_ledger_t m_out
);
    import FdeTypes::*;
    fde_ledger_t internal_spectrum;
    mass_t total_mass;
    localparam mass_t SIGMA_BN = 0.450000;
    localparam mass_t OMEGA_B  = 1.000000;
    localparam mass_t OMEGA_N  = 4.000000;
    localparam mass_t GAMMA_C  = 0.500000;
    localparam mass_t CSHARE   = 0.500000;

    always_ff @(posedge clk) begin
        if (gamma_field < GAMMA_C) begin
            mass_t d = SIGMA_BN*((m_in.B/OMEGA_B) - (m_in.N/OMEGA_N));
            internal_spectrum.N <= m_in.N + (1.0 - CSHARE)*d;
            internal_spectrum.B <= m_in.B - d;
            internal_spectrum.T <= m_in.T + 0.5*CSHARE*d;
            internal_spectrum.F <= m_in.F + 0.5*CSHARE*d;
        end else begin
            total_mass = internal_spectrum.T + internal_spectrum.F +
                         internal_spectrum.B + internal_spectrum.N;
            m_out.B <= total_mass;
            m_out.T <= 0.0; m_out.F <= 0.0; m_out.N <= 0.0;
        end
    end
endmodule
