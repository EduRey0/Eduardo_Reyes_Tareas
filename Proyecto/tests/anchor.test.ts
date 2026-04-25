describe("listar tareas pro", () => {
  it("crear varias tareas y listarlas por owner", async () => {
    const owner = pg.wallet.publicKey;

    for (let i = 0; i < 3; i++) {
      const id = new BN(Date.now() + i);

      const [taskPda] = web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from("task2"),
          owner.toBuffer(),
          id.toArrayLike(Buffer, "le", 8),
        ],
        pg.program.programId
      );

      const tx = await pg.program.methods
        .crear(id, `Tarea ${i}`)
        .accounts({
          owner,
          task: taskPda,
          systemProgram: web3.SystemProgram.programId,
        })
        .rpc();

      console.log(`CREATE ${i}:`, tx);
    }

    const cuentasRaw = await pg.connection.getProgramAccounts(
      pg.program.programId
    );

    const cuentasValidas = [];

    for (const c of cuentasRaw) {
      try {
        const decoded = pg.program.coder.accounts.decode(
          "Task",
          c.account.data
        );

        if (decoded.owner.toBase58() === owner.toBase58()) {
          cuentasValidas.push({
            publicKey: c.pubkey,
            account: decoded,
          });
        }
      } catch (e) {
        // Ignora cuentas viejas (IMPORTANTE)
      }
    }


    console.log("\n📦 TAREAS ENCONTRADAS:");
    console.log("Total:", cuentasValidas.length);

    cuentasValidas.forEach((c, i) => {
      console.log(`\n--- Tarea ${i} ---`);
      console.log("PDA:", c.publicKey.toBase58());
      console.log("ID:", c.account.id.toString());
      console.log("Título:", c.account.titulo);
      console.log("Completado:", c.account.completado);
    });

    if (cuentasValidas.length < 3) {
      throw new Error("No se crearon todas las tareas");
    }
  });
});