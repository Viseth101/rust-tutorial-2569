trait Payment {
 fn pay(&self);
}
struct CreditCard;
struct Cash;
struct QRCode;

impl Payment for CreditCard {
 fn pay(&self) {
     println!("Pay with Credit Card");
    }
}

impl Payment for Cash {
    fn pay(&self) {
        println!("Pay with Cash");
    }
}

impl Payment for QRCode {
    fn pay(&self) {
        println!("Pay with QR Code");
    }
}


fn main() {
    let credit_card = CreditCard;
    let cash = Cash;
    let qr_code = QRCode;
    credit_card.pay();
    cash.pay();
    qr_code.pay();
}
