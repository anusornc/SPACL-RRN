# แนะนำ Ontology มาตรฐานสำหรับการทดสอบ Enhanced Reasoner

ตามการวิจัยและการสำรวจแหล่งข้อมูลมาตรฐาน ขอเสนอ ontology มาตรฐานที่ใช้ในการทดสอบ OWL reasoner ซึ่งจะช่วยให้การเปรียบเทียบประสิทธิภาพมีความน่าเชื่อถือและเป็นที่ยอมรับในชุมชนวิจัย

## 1. ORE Competition Benchmark Suite

**OWL Reasoner Evaluation (ORE) Competition** เป็นการแข่งขันประจำปีที่ประเมิน OWL 2 reasoner ต่างๆ โดยใช้ชุดข้อมูลมาตรฐาน

### คุณลักษณะ:
- ชุดข้อมูลขนาดใหญ่ที่มีความซับซ้อนหลากหลาย
- อัปเดตทุกปีสำหรับการแข่งขัน
- ครอบคลุม OWL 2 profiles และระดับการแสดงออกต่างๆ
- มีการแบ่งประเภทตาม problem type เช่น "effectively propositional" หรือ "OWL 2 EL ontology"

### การเข้าถึง:
- เว็บไซต์: https://www.w3.org/community/owled/ore-2015-workshop
- Archive: http://dl.kr.org/dl2015/

## 2. LUBM (Lehigh University Benchmark)

**Lehigh University Benchmark** เป็น benchmark ที่ได้รับการยอมรับอย่างกว้างขวางสำหรับการประเมิน Semantic Web repositories

### คุณลักษณะ:
- **Domain:** University domain ontology
- **ขนาด:** ปรับขนาดได้ตามต้องการ
- **ข้อมูล:** Synthetic data ที่สามารถทำซ้ำได้
- **Test Queries:** 14 test queries ในรูปแบบ SPARQL 1.0

### ไฟล์ที่สำคัญ:
- **Ontology:** Univ-Bench OWL Version
- **Data Generator:** UBA1.7 (รองรับทั้ง OWL และ DAML+OIL)
- **Test Module:** UBT1.1

### การดาวน์โหลด:
- เว็บไซต์หลัก: https://swat.cse.lehigh.edu/projects/lubm/
- Repository: SemWebCentral
- ข้อมูลอ้างอิง: สามารถใช้ options `-index 0 -seed 0` เพื่อสร้างชุดข้อมูลเดียวกับที่ใช้ในงานวิจัย

## 3. Gene Ontology (GO)

**Gene Ontology** เป็น ontology ขนาดใหญ่ที่ใช้ในด้าน life sciences และเป็นตัวอย่างที่ดีของ real-world ontology

### เวอร์ชันที่แนะนำ:

#### 3.1 go-basic.owl
- **คุณลักษณะ:** Acyclic ontology ที่ปลอดภัยสำหรับ annotation propagation
- **Relations:** is_a, part_of, regulates, negatively_regulates, positively_regulates
- **แนะนำสำหรับ:** เครื่องมือ annotation ส่วนใหญ่
- **ดาวน์โหลด:** https://geneontology.org/docs/download-ontology/

#### 3.2 go.owl
- **คุณลักษณะ:** Core ontology ที่มี relationships เพิ่มเติม
- **Relations:** รวม has_part และ occurs_in
- **หมายเหตุ:** อาจสร้าง cycles ในออนโทโลยี

#### 3.3 go-plus.owl
- **คุณลักษณะ:** Fully axiomatised version
- **รวม:** Cross-ontology relationships และ imports จาก ChEBI, Cell Ontology, Uberon
- **ขนาด:** ใหญ่ที่สุดและซับซ้อนที่สุด

## 4. Biomedical Ontologies

### 4.1 SNOMED CT
- **คุณลักษณะ:** Clinical terminology ontology ขนาดใหญ่
- **รูปแบบ:** OWL 2 format
- **การเข้าถึง:** ผ่าน NCBO BioPortal หรือ IHTSDO
- **เครื่องมือ:** SNOMED OWL Toolkit

### 4.2 National Cancer Institute Thesaurus (NCIT)
- **คุณลักษณะ:** Cancer domain ontology
- **ขนาด:** มีคลาสและคุณสมบัติจำนวนมาก
- **การใช้งาน:** ทดสอบ reasoner กับ domain-specific knowledge

### 4.3 Human Phenotype Ontology (HPO)
- **คุณลักษณะ:** Human phenotype descriptions
- **รูปแบบ:** OWL format
- **ความซับซ้อน:** Medium ถึง High

## 5. W3C OWL 2 Test Cases

### คุณลักษณะ:
- **Official conformance tests** จาก W3C
- ครอบคลุม **ทุกฟีเจอร์** ของ OWL 2
- รวม **positive และ negative entailment tests**
- เหมาะสำหรับทดสอบความถูกต้องของ reasoner

### การเข้าถึง:
- W3C OWL 2 Conformance: https://www.w3.org/TR/owl2-conformance/

## 6. OWL 2 EL Profile Ontologies

สำหรับทดสอบ reasoner ที่เฉพาะเจาะจงกับ EL profile:

### 6.1 Large-scale Life Science Ontologies
- **Gene Ontology** (รุ่น EL-compatible)
- **SNOMED CT** (บางส่วน)
- **Chemical Entities of Biological Interest (ChEBI)**

### 6.2 คุณลักษณะ:
- **Tractable reasoning complexity**
- **Large-scale real-world data**
- เหมาะสำหรับทดสอบ **scalability**

## คำแนะนำการใช้งาน

### สำหรับการทดสอบเบื้องต้น:
1. **LUBM** - เริ่มต้นด้วยขนาดเล็ก (1-5 universities)
2. **go-basic.owl** - ทดสอบกับ real-world ontology
3. **W3C Test Cases** - ตรวจสอบความถูกต้อง

### สำหรับการทดสอบประสิทธิภาพ:
1. **LUBM** - เพิ่มขนาดเป็น 50-1000 universities
2. **go-plus.owl** - ทดสอบกับ complex axioms
3. **SNOMED CT** - ทดสอบกับ large-scale clinical data

### สำหรับการเปรียบเทียบกับ reasoner อื่น:
1. ใช้ **ORE Competition benchmark suite**
2. รายงานผลตาม **ORE metrics**
3. เปรียบเทียบกับ **published results**

## สรุป

การใช้ ontology มาตรฐานเหล่านี้จะช่วยให้การประเมินประสิทธิภาพของ Enhanced Reasoner มีความน่าเชื่อถือและสามารถเปรียบเทียบกับงานวิจัยอื่นๆ ได้อย่างมีประสิทธิภาพ แนะนำให้เริ่มจากชุดข้อมูลขนาดเล็กก่อน แล้วค่อยเพิ่มความซับซ้อนและขนาดตามลำดับ
