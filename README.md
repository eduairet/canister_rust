# canister_rust

Prueba de canister en rust usando las [guías de Internet Computer](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)

## Prueba local

-   Probar proyecto:

    ```bash
    dfx start --background
    dfx deploy
    ```

    -   Navegador:
        ```SHELL
        canister_rust_backend: http://127.0.0.1:4943/?canisterId=be2us-64aaa-aaaaa-qaabq-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
        multiply_deps: http://127.0.0.1:4943/?canisterId=be2us-64aaa-aaaaa-qaabq-cai&id=bd3sg-teaaa-aaaaa-qaaba-cai
        ```
    -   Terminal:
        ```SHELL
        dfx canister call multiply_deps read # (1 : nat)
        dfx canister call multiply_deps mul '(3)' # (9 : nat)
        ```

-   Actualizar canister

    ```bash
    dfx build
    dfx canister install --all --mode upgrade
    ```
