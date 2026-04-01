from enum import Enum

class VendingMachineState(Enum):
    IDLE = 0
    HAS_MONEY = 1
    DISPENSING = 2
    OUT_OF_STOCK = 3
    MAINTENANCE = 4

class VendingMachine:
    def __init__(self):
        self.state = VendingMachineState.IDLE
        self.stock = 3

    def handle_state(state):
        if state == VendingMachineState.IDLE:
            print("Vending Machine is awaiting payment")
        elif state == VendingMachineState.HAS_MONEY:
            print("Please enter item number")
        elif state == VendingMachineState.DISPENSING:
            print("Dispensing item...")
        elif state == VendingMachineState.OUT_OF_STOCK:
            print("Item is out of stock")
        elif state == VendingMachineState.MAINTENANCE:
            print("Vending Machine is under maintenance")

    def insert_money(self):
        if self.state == VendingMachineState.IDLE:
            self.state = VendingMachineState.HAS_MONEY
            print("Money accepted.")
        elif self.state == VendingMachineState.OUT_OF_STOCK:
            print("Cannot accept money: machine is out of stock.")
        elif self.state == VendingMachineState.MAINTENANCE:
            print("Cannot accept money: machine is under maintenance.")
        else:
            print("Money already inserted or item is being dispensed.")

    def select_item(self):
        if self.state == VendingMachineState.HAS_MONEY:
            if self.stock > 0:
                self.state = VendingMachineState.DISPENSING
                print("Item selected.")
                self.dispense_item()
            else:
                self.state = VendingMachineState.OUT_OF_STOCK
                print("Cannot dispense item: out of stock.")
        elif self.state == VendingMachineState.IDLE:
            print("Insert money first.")
        elif self.state == VendingMachineState.MAINTENANCE:
            print("Cannot select item: machine is under maintenance.")
        elif self.state == VendingMachineState.OUT_OF_STOCK:
            print("Cannot select item: machine is out of stock.")
        else:
            print("Item is already being dispensed.")

    def dispense_item(self):
        if self.state == VendingMachineState.DISPENSING:
            self.stock -= 1
            print("Item dispensed.")

            if self.stock == 0:
                self.state = VendingMachineState.OUT_OF_STOCK
            else:
                self.state = VendingMachineState.IDLE
        else:
            print("Machine is not ready to dispense.")

    def restock(self, amount):
        if amount > 0:
            self.stock += amount
            self.state = VendingMachineState.IDLE
            print("Machine restocked.")
        else:
            print("Restock amount must be greater than 0.")

    def set_maintenance(self):
        self.state = VendingMachineState.MAINTENANCE
        print("Machine set to maintenance mode.")

    def end_maintenance(self):
        if self.stock > 0:
            self.state = VendingMachineState.IDLE
        else:
            self.state = VendingMachineState.OUT_OF_STOCK
        print("Maintenance mode ended.")


machine = VendingMachine()

machine.handle_state()
machine.insert_money()
machine.handle_state()
machine.select_item()
machine.handle_state()

machine.insert_money()
machine.select_item()
machine.insert_money()
machine.select_item()
machine.handle_state()

machine.insert_money()

machine.restock(5)
machine.handle_state()

machine.set_maintenance()
machine.insert_money()
machine.handle_state()

machine.end_maintenance()
machine.handle_state()