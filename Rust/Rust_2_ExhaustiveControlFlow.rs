/******************************************************************************

Rust_2_ExhaustiveControlFlow

This program shows how Rust uses match to handle every enum variant explicitly. 
It also demonstrates that if a variant is missing—or if a new variant is added later 
without updating the match—Rust prevents the program from compiling, enforcing exhaustive control flow.
*******************************************************************************/
#[derive(Debug)]
enum PaymentStatus {
    Pending,
    Paid,
    Failed, 
    //Processing,
}
// comment out one of the variants above^ , run, then add a variant, run to test Exhaustiveness

fn describe_status(status: PaymentStatus) {
    match status {
        PaymentStatus::Pending => println!("Payment is still being processed."),
        PaymentStatus::Paid => println!("Payment completed successfully."),
        PaymentStatus::Failed => println!("Payment failed."),
    }
}

fn main() {
    println!("Program 2: Exhaustive Control Flow in Rust\n");

    // Demonstrate that all enum variants are handled
    describe_status(PaymentStatus::Pending);
    describe_status(PaymentStatus::Paid);
    describe_status(PaymentStatus::Failed);

    println!("\nConclusion:");
    println!("- Rust requires match statements on enums to be exhaustive.");
    println!("- Because all variants were handled, the program compiled and ran successfully.");
    println!("- If even one variant is omitted, Rust fails at compile time.");
    println!("- If a new variant is later added to the enum but the match is not updated, Rust also fails at compile time.");
}