import * as anchor from "@project-serum/anchor";
import { expect } from 'chai';
import { Program } from "@project-serum/anchor";
import { AnchorTicTacToe } from "../target/types/anchor_tic_tac_toe";

async function play(program, game, player, tile, expectedTurn, expectedGameState, expectedBoard) {
  await program.rpc.play(tile, {
    accounts: {
      player: player.publicKey,
      game
    },
    signers: player instanceof (anchor.Wallet as any) ? [] : [player]
  });

  const gameState = await program.account.game.fetch(game);
  expect(gameState.turn).to.equal(expectedTurn);
  expect(gameState.state).to.eql(expectedGameState);
  expect(gameState.board).to.eql(expectedBoard);
}

describe("anchor-tic-tac-toe", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.AnchorTicTacToe as Program<AnchorTicTacToe>;

  it('setup game!', async() => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const playerOne = program.provider.wallet;
    const playerTwo = anchor.web3.Keypair.generate();
    await program.rpc.setupGame(playerTwo.publicKey, {
      accounts: {
        game: gameKeypair.publicKey,
        playerOne: playerOne.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [gameKeypair]
    });

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.turn).to.equal(1);
    expect(gameState.players).to.eql([playerOne.publicKey, playerTwo.publicKey]);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board).to.eql([
      [null, null, null],
      [null, null, null],
      [null, null, null],
      ]);
  });

  it('player one wins', async() => {
    const gameKeypair = anchor.web3.Keypair.generate();
    const playerOne = program.provider.wallet;
    const playerTwo = anchor.web3.Keypair.generate();
    await program.rpc.setupGame(playerTwo.publicKey, {
      accounts: {
        game: gameKeypair.publicKey,
        playerOne: playerOne.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [gameKeypair]
    });

    let gameState = await program.account.game.fetch(gameKeypair.publicKey);
    expect(gameState.turn).to.equal(1);
    expect(gameState.players).to.eql([playerOne.publicKey, playerTwo.publicKey]);
    expect(gameState.state).to.eql({ active: {} });
    expect(gameState.board).to.eql([
      [null, null, null],
      [null, null, null],
      [null, null, null]
    ]);

    await play(
      program,
      gameKeypair.publicKey,
      playerOne,
      {row: 0, column: 0},
      2,
      { active: {} },
      [
        [{x:{}}, null, null],
        [null, null, null],
        [null, null, null]
      ]
    );
    await play(
      program,
      gameKeypair.publicKey,
      playerTwo,
      {row: 1, column: 0},
      3,
      { active: {} },
      [
        [{x:{}}, null, null],
        [{o:{}}, null, null],
        [null, null, null]
      ]
    );
    await play(
      program,
      gameKeypair.publicKey,
      playerOne,
      {row: 0, column: 1},
      4,
      { active: {} },
      [
        [{x:{}}, {x:{}}, null],
        [{o:{}}, null, null],
        [null, null, null]
      ]
    );
    await play(
      program,
      gameKeypair.publicKey,
      playerTwo,
      {row: 1, column: 1},
      5,
      { active: {} },
      [
        [{x:{}}, {x:{}}, null],
        [{o:{}}, {o:{}}, null],
        [null, null, null]
      ]
    );
    await play(
      program,
      gameKeypair.publicKey,
      playerOne,
      {row: 0, column: 2},
      5,
      { won: { winner: playerOne.publicKey } },
      [
        [{x:{}}, {x:{}}, {x:{}}],
        [{o:{}}, {o:{}}, null],
        [null, null, null]
      ]
    );
  });


});
