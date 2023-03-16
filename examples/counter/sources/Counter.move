module counter::counter {
    use sui::tx_context::TxContext;
    use sui::object::{Self, UID};
    use sui::transfer;

    struct Counter has key {
        id: UID,
        value: u64
    }

    fun init(ctx: &mut TxContext) {
        transfer::share_object(Counter { 
            id: object::new(ctx), 
            value: 0
        })
    }

    public entry fun increment(counter: &mut Counter) {
        counter.value = counter.value + 1
    }

    public entry fun decrement(counter: &mut Counter) {
        counter.value = counter.value - 1
    }
}