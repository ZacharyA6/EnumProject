/******************************************************************************

5_Rust_AppliedEnum

This implementation of a Ticket System ensures type safety and handling of errors efficiently with enums
enums provide compile-time guarantees that only valid states can exist, 
and Rust’s match ensures the logic handles every case explicitly. Replacing them with strings 
would remove type safety and require manual validation throughout the program. 

*******************************************************************************/
#[derive(Debug)]
enum TicketStatus {
    New,
    InProgress,
    Escalated,
    Resolved,
    Closed,
}

#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
enum Department {
    TechnicalSupport,
    Billing,
    Security,
}

#[derive(Debug)]
struct Ticket {
    id: u32,
    status: TicketStatus,
    priority: Priority,
    department: Department,
}

fn assign_response_time(priority: &Priority) -> u32 {
    match priority {
        Priority::Low => 72,
        Priority::Medium => 24,
        Priority::High => 8,
        Priority::Critical => 1,
    }
}

fn route_ticket(ticket: &Ticket) {
    match ticket.department {
        Department::TechnicalSupport => {
            println!("Route ticket #{} to Technical Support queue.", ticket.id)
        }
        Department::Billing => {
            println!("Route ticket #{} to Billing queue.", ticket.id)
        }
        Department::Security => {
            println!("Route ticket #{} to Security Incident Response.", ticket.id)
        }
    }
}

fn process_ticket(ticket: &mut Ticket) {
    println!("\nProcessing ticket #{}", ticket.id);
    println!("Current status: {:?}", ticket.status);
    println!("Priority: {:?}", ticket.priority);
    println!("Department: {:?}", ticket.department);

    let response_time = assign_response_time(&ticket.priority);
    println!("Required response time: {} hour(s)", response_time);

    route_ticket(ticket);

    // Enum-driven business logic:
    match (&ticket.status, &ticket.priority, &ticket.department) {
        (TicketStatus::New, Priority::Critical, Department::Security) => {
            println!("Critical security issue detected: auto-escalating immediately.");
            ticket.status = TicketStatus::Escalated;
        }
        (TicketStatus::New, _, _) => {
            println!("Ticket is new: moving to InProgress.");
            ticket.status = TicketStatus::InProgress;
        }
        (TicketStatus::InProgress, Priority::Critical, _) => {
            println!("Critical issue still in progress: escalate to senior team.");
            ticket.status = TicketStatus::Escalated;
        }
        (TicketStatus::InProgress, _, _) => {
            println!("Work completed: marking ticket as Resolved.");
            ticket.status = TicketStatus::Resolved;
        }
        (TicketStatus::Escalated, _, _) => {
            println!("Escalated issue reviewed: marking ticket as Resolved.");
            ticket.status = TicketStatus::Resolved;
        }
        (TicketStatus::Resolved, _, _) => {
            println!("Customer confirmed fix: closing ticket.");
            ticket.status = TicketStatus::Closed;
        }
        (TicketStatus::Closed, _, _) => {
            println!("Ticket is already closed. No further action.");
        }
    }

    println!("New status: {:?}", ticket.status);
}

fn main() {
    let mut ticket1 = Ticket {
        id: 1001,
        status: TicketStatus::New,
        priority: Priority::Critical,
        department: Department::Security,
    };

    let mut ticket2 = Ticket {
        id: 1002,
        status: TicketStatus::New,
        priority: Priority::Medium,
        department: Department::Billing,
    };

    let mut ticket3 = Ticket {
        id: 1003,
        status: TicketStatus::InProgress,
        priority: Priority::High,
        department: Department::TechnicalSupport,
    };

    let mut ticket4 = Ticket {
    id: 1004,
    status: TicketStatus::New,
    priority: Priority::Low,
    department: Department::TechnicalSupport,
    };
    
    process_ticket(&mut ticket1);
    process_ticket(&mut ticket2);
    process_ticket(&mut ticket3);
    process_ticket(&mut ticket4);
    // Process ticket1 again to show state progression
    process_ticket(&mut ticket1);
    process_ticket(&mut ticket1);
}
