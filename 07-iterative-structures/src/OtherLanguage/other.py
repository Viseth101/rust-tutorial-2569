numbers = [10, 20, 30]

# 1. วนลูปผ่าน List ใน Python (ใช้ Iterable protocol)
for num in numbers:
    print(f"Number: {num}")

# 2. การหาผลลัพธ์จากลูปต้องใช้ตัวแปรภายนอกมารับค่า (ไม่มี loop expression)
count = 0
while True:
    count += 1
    if count == 5:
        result = count * 2
        break
print(f"Result from loop: {result}")

# 3. ตัวอย่างความเสี่ยงของการแอบแก้ไข List ระหว่างวนลูปใน Python (เกิดข้อผิดพลาดตอน Runtime)
for num in numbers:
    numbers.remove(num)  # ข้อมูลจะถูกลบข้ามองค์ประกอบไปเรื่อยๆ โดยที่คอมไพเลอร์ไม่เตือนก่อนรัน