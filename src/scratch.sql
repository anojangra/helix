SELECT
    chromosome,
    kelly,
    cum_pnl,
    w_kelly,
    num_of_trades,
    generation
FROM trade_chromosomes
WHERE num_of_trades > 500
GROUP BY chromosome, kelly, cum_pnl, w_kelly, num_of_trades, generation
ORDER BY cum_pnl DESC;