# Increments smart contract using substrate Ink 

https://docs.substrate.io/tutorials/v3/ink-workshop/pt2/

Test the smart contract
```shell
$ cargo +nightly test
```

Build the smart contract
```shell
$ cargo +nightly contract build
```

## Start node with pallet contract

```shell
$ substrate-contracts-node --dev --tmp
```

## Open hosted UI

Also test in Canvas UI:

https://docs.substrate.io/tutorials/v3/ink-workshop/pt2/

## Upload contract to node

- Click the Upload & Instantiate Contract button.
- Choose an Instantiation account (e.g. ALICE).
- Give the contract a descriptive Name (e.g. Flipper Contract).
- Drag the flipper.contract file that contains the bundled Wasm blob and metadata into the drag & drop area. You will see the UI parse the metadata and showing what functions the contract contains.
- Click the Constructor Details

## Lazy storage

This loads its value from storage upon first use.

https://paritytech.github.io/ink/ink_storage/struct.Lazy.html

Use this if the storage field does not need to be loaded in some or most cases.

## HashMap

A hash map operating on the contract storage.

https://paritytech.github.io/ink/ink_storage/collections/hashmap/struct.HashMap.html

Stores a mapping between keys and values.

Usually AccountId and value.