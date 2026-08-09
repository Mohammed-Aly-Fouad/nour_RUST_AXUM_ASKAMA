-- ========================================================
-- Insert Product Variants Seed Data
-- ========================================================
INSERT INTO public.product_variants (product_id, brand_id, sku, name, name_ar, stock_quantity, attr) VALUES 
(
  (SELECT id FROM public.products WHERE name = 'Glue stick'),
  (SELECT id FROM public.brands WHERE name = 'PASCO'),
  'PAS-GLU-08G', 'Pasco Glue Stick 8g', 'جلو ستيك باسك 8 جرام', 29, 
  '{"Uses": "Paper", "Weight": "8g", "Material": "PVP Solid Glue"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name = 'FABER CASTLE'),
  'FAB-2122-2B', 'Faber-Castell 2122 2B Pencil', 'قلم رصاص فابر كاستل 2122 2B', 72, 
  '{"Model": "2122", "Degree": "2B", "Eraser": "false", "Shape": "Hexagonal"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pocket Notebook'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-FCLAY-12C', 'Everygreen Modeling Clay 12 Colors', 'صلصال افيفرجرين 12 لون', 1, 
  '{"Age": "3+", "Size": "Small", "Exp Date": "Oct 2028", "Colors Count": "12"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Sketchbook'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-RBAND-RED', 'Everygreen Rubber Bands Red', 'أساتك افيفرجرين أحمر', 10, 
  '{"Color": "Red", "Country": "Thailand", "Usage": "Money / General"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Sketchbook'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-RBAND-GRN', 'Everygreen Rubber Bands Green', 'أساتك افيفرجرين أخضر', 13, 
  '{"Color": "Green", "Country": "Thailand", "Usage": "Money / General"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Bulldog Clip'),
  (SELECT id FROM public.brands WHERE name = 'FLOWER'),
  'FLW-INK-S-BLU', 'Flower Stamp Pad Ink Blue 30ml', 'حبر ختامة فلاور أزرق 30 مل', 9, 
  '{"Color": "Blue", "Volume": "30 ml"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Protractor'),
  (SELECT id FROM public.brands WHERE name = 'ETMAN GROUP'),
  'OTM-GGSTK-L', 'Otima Glue Gun Stick Large Clear', 'غيار مسدس شمع أوتيما كبير شفاف', 27, 
  '{"Color": "Clear", "Length": "19.5 cm", "Size": "Large"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Thermal Paper Roll'),
  (SELECT id FROM public.brands WHERE name = 'POWER'),
  'PWR-PLBL-1L', 'Power Price Labels 1 Line Roll', 'ملصقات أسعار باور 1 خط بكرة', 10, 
  '{"Format": "Roll", "Usage": "Price Gun (1 Line)"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Legal Contract'),
  (SELECT id FROM public.brands WHERE name = 'ETMAN GROUP'),
  'OTM-RBN-4CM-MIX', 'Otima Wrapping Ribbon 4cm Multi-color', 'شريط لف هدايا أوتيما 4 سم متعدد الألوان', 15, 
  '{"Color": "Multi-color", "Width": "4cm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name = 'MIDGO'),
  'MDG-PDB-A4-10P-MIX', 'M&G Pocket Display Book A4 10 Pockets', 'هولدر جيوب ام اند جي A4 10 جيوب', 5, 
  '{"Item Number": "A-10", "Pockets": "10", "Size": "A4", "Color": "Multi-color"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name = 'MIDGO'),
  'MDG-PDB-A4-20P-MIX', 'M&G Pocket Display Book A4 20 Pockets', 'هولدر جيوب ام اند جي A4 20 جيب', 3, 
  '{"Item Number": "A-20", "Pockets": "20", "Size": "A4", "Color": "Multi-color"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name = 'MIDGO'),
  'MDG-PDB-A4-30P-MIX', 'M&G Pocket Display Book A4 30 Pockets', 'هولدر جيوب ام اند جي A4 30 جيب', 3, 
  '{"Item Number": "M-230A", "Pockets": "30", "Size": "A4", "Color": "Multi-color"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'U-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name = 'DL_DINGLI'),
  'DL-PDB-A4-40P-MIX', 'Deli Pocket Display Book A4 40 Pockets', 'هولدر جيوب ديلي A4 40 جيب', 3, 
  '{"Model": "DL5324(FASHION)", "Pockets": "40", "Color": "Multi-color", "Szie": "A4"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Eraser'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-COR-PEN-6ML', 'Everygreen Correction Pen 6ml', 'قلم كوريكتور افيفرجرين 6 مل', 12, 
  '{"Color": "White", "Model": "CP331", "Volume": "6ml", "Exp Date": "2030-01-01"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Eraser'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-COR-PEN-8ML', 'Everygreen Correction Pen 8ml', 'قلم كوريكتور افيفرجرين 8 مل', 10, 
  '{"Color": "White", "Model": "CP391", "Volume": "8ml", "Exp Date": "2030-01-01"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Clipboard'),
  (SELECT id FROM public.brands WHERE name = 'ELUCKY_EVERLUCKY'),
  'ELK-BDC-25MM-BLK', 'El-Keshky Bulldog Clip 25mm Black', 'مشبك ورق الكشكي 25 مم أسود', 144, 
  '{"Color": "Black", "Material": "Metal", "Size": "25mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Rubber Bands'),
  (SELECT id FROM public.brands WHERE name = 'SOFI PLAST'),
  'SFP-PRG-FL-A4-MIX', 'Safari Report Cover A4 Prong Multi-color', 'دوسيه تقارير سفاري A4 بنز متعدد الألوان', 25, 
  '{"Binding": "Prong", "Color": "Multi-color", "Material": "Plastic", "Size": "A4"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Sticky Note'),
  (SELECT id FROM public.brands WHERE name = 'SEMA'),
  'SMA-CLB-A4-SGL', 'Samir & Aly Clipboard A4 Single', 'بلانشيطة سمير وعلى A4 مفرد', 6, 
  '{"Color": "Multi-color", "Material": "Cardboard", "Size": "A4", "Subtype": "Single"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Sticky Note'),
  (SELECT id FROM public.brands WHERE name = 'BRAVO'),
  'BRV-CLB-A4-PEN', 'Bravo Clipboard A4 with Pen Holder', 'بلانشيطة برافو A4 بالحامل والقلم', 4, 
  '{"Color": "Multi-color", "Includes": "Pen", "Material": "Cardboard", "Size": "A4", "Subtype": "Single"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Sticky Note'),
  (SELECT id FROM public.brands WHERE name = 'DIGITAL'),
  'DGT-CLB-A4-PVC', 'Digital Clipboard A4 PVC Coated', 'بلانشيطة ديجيتال A4 كرتون مضغوط', 2, 
  '{"Color": "Multi-color", "Material": "PVC Coated", "Size": "A4", "Subtype": "Single"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'L-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name = 'SOFI PLAST'),
  'SFP-USH-A4-95MIC', 'Safari U-Shape Protector A4 95 Micron', 'حافظة حرف يو سفاري A4 95 مايكرون', 46, 
  '{"Color": "Clear", "Holes": "11", "Material": "PP", "Size": "A4", "Texture": "Embossed", "Thickness": "95 Micron"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'L-Shape Sheet Protector'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-USHP-A4-11H-LT', 'Everygreen U-Shape Protector A4 Light', 'حافظة حرف يو افيفرجرين A4 خفيفة', 25, 
  '{"Color": "Clear", "Holes": "11", "Material": "PP", "Size": "A4", "Texture": "Smooth", "Thickness": "Light"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Graphite Pencil'),
  (SELECT id FROM public.brands WHERE name = 'DOMS'),
  'DMS-SHP-SML-PL', 'Doms Single Hole Pencil Sharpener Small', 'براية دومس فتحة واحدة صغيرة', 100, 
  '{"Holes": "Single Hole", "Material": "Plastic", "Model": "7910", "Size": "Small"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Highlighter'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-PMRK-BLU-CHS', 'Rotring Permanent Marker Blue Chisel', 'قلم ماركر دائم روتريتو أزرق مشطوف', 21, 
  '{"Color": "Blue", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Highlighter'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-PMRK-RED-CHS', 'Rotring Permanent Marker Red Chisel', 'قلم ماركر دائم روتريتو أحمر مشطوف', 24, 
  '{"Color": "Red", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Highlighter'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-PMRK-BLK-CHS', 'Rotring Permanent Marker Black Chisel', 'قلم ماركر دائم روتريتو أسود مشطوف', 12, 
  '{"Color": "Black", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pen'),
  (SELECT id FROM public.brands WHERE name = 'MICRO'),
  'MIC-MPEN-0p7-2114', 'Micro Mechanical Pencil 0.7mm', 'قلم رصاص سنون مايكرو 0.7 مم', 23, 
  '{"Body Material": "Plastic", "Color": "Multi-color", "Lead_Size": "0.7mm", "Model": "2114-1M"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pocket Display Book'),
  (SELECT id FROM public.brands WHERE name = 'FABER CASTLE'),
  'FAB-ERS-RED-189577', 'Faber-Castell Eraser Red Small 189577', 'استيكة فابر كاستل صغيرة أحمر 189577', 30, 
  '{"Color": "Red", "Model": "189577", "Shape": "Rectangular", "Size": "Small"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pocket Display Book'),
  (SELECT id FROM public.brands WHERE name = 'DOMS'),
  'DOM-ERS-WHT-3421', 'Doms Eraser Dust Free White Small', 'استيكة دومس خالية من الغبار أبيض صغيرة', 17, 
  '{"Color": "White", "Material": "Dust Free", "Model": "[8172,3421]", "Size": "Small"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Compass'),
  (SELECT id FROM public.brands WHERE name = 'QUEEN'),
  'QEN-PRT-CLR-180', 'Queen Protractor 180 Degree Clear', 'منقلة كوين 180 درجة شفاف', 36, 
  '{"Color": "Clear", "Degree": "180", "Material": "Plastic", "Model": "G-1001"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name = 'MICHAEL'),
  'MCH-RLR-MTL-30CM', 'Mitchell Metal Ruler 30cm Heavy', 'مسطرة معدن ميتشيل 30 سم ثقيلة', 30, 
  '{"Material": "Metal", "Weight": "Heavy", "Width": "25mm", "length": "30cm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name = 'CHANYI'),
  'CY-RLR-PLS-30-9121', 'Chuangyi Plastic Ruler 30cm Clear', 'مسطرة بلاستيك تشوانجي 30 سم شفاف', 24, 
  '{"Color": "Clear", "Material": "Plastic", "Model": "CY9121", "Width": "30mm", "Length": "30cm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Report Cover'),
  (SELECT id FROM public.brands WHERE name = 'ALWARRAK'),
  'WRK-TPR-57MM', 'World Thermal Paper Roll 57mm White', 'بكر حراري ورلد 57 مم أبيض', 28, 
  '{"Color": "White", "Width": "57mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pencil Case'),
  (SELECT id FROM public.brands WHERE name = 'GAOERJIEFU'),
  'GAO-BAL-MET-10IN-20P', 'Gaole Metallic Balloon 10 Inch 20 Pcs', 'بالون ميتاليك جول 10 بوصة 20 قطعة', 5, 
  '{"Design": "Printed", "Finish": "Metalic", "Pack Quantity": "20", "Size": "10 inch"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-RLR-GRD-20CM', 'Prima Grid Ruler 20cm Clear', 'مسطرة مربعات بريما 20 سم شفاف', 22, 
  '{"Color": "Clear", "Design": "Grids", "Length": "20cm", "Material": "Plastic", "Width": "25mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Lease agreement'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-RLR-GRD-30CM', 'Prima Grid Ruler 30cm Clear', 'مسطرة مربعات بريما 30 سم شفاف', 22, 
  '{"Color": "Clear", "Design": "Grids", "Length": "30cm", "Material": "Plastic", "Width": "25mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Modeling Clay'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'ROT-HLT-BRT-4C', 'Rotring Highlighter Bright 4 Colors Pack', 'طقم أقلام هاي لايتر روتريتو 4 ألوان', 2, 
  '{"Model": "Bright", "Pack Quantity": "4", "Tip Size": "2mm*5mm", "Tip Type": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name = 'DELI'),
  'DEL-PNC-TRG-37013-2B', 'Deli Triangular Pencil 2B 37013', 'قلم رصاص ديلي مثلث 2B 37013', 45, 
  '{"Lead Grade": "2B", "Material": "Wood", "Model": "37013", "Shape": "Triangular"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Wrapping Ribbon'),
  (SELECT id FROM public.brands WHERE name = 'QUEEN'),
  'VEN-RLR-SHP-20CM-2400', 'Queen Venn Stencil Ruler 20cm', 'مسطرة أشكال فين كوين 20 سم', 24, 
  '{"Color": "Clear Multi-color", "Design": "Venn Diagram", "Length": "20cm", "Material": "Plastic", "Model": "2400", "Width": "50mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pen'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-MCL-0p5MM', 'Prima Mechanical Pencil 0.5mm Stand', 'قلم رصاص سنون بريما 0.5 مم', 4, 
  '{"Color": "Multi-color", "Lead Size": "0.5mm", "Model": "On Stand"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pen'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-MCL-0p7MM', 'Prima Mechanical Pencil 0.7mm Stand', 'قلم رصاص سنون بريما 0.7 مم', 12, 
  '{"Color": "Multi-color", "Lead Size": "0.7mm", "Material": "Plastic", "Model": "On Stand"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pen'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-MCL-0p9MM', 'Prima Mechanical Pencil 0.9mm Stand', 'قلم رصاص سنون بريما 0.9 مم', 8, 
  '{"Color": "Multi-color", "Lead Size": "0.9", "Material": "Plastic", "Model": "On Stand"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'ROT-LB-0p7MM-BLU', 'Rotring Liquid Ball Pen 0.7mm Blue', 'قلم جاف ليكويد روتريتو 0.7 مم أزرق', 21, 
  '{"Color": "Blue", "Ink Type": "Liquid Ink", "Model": "Liquid Ball", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'ROT-LB-0p7MM-RED', 'Rotring Liquid Ball Pen 0.7mm Red', 'قلم جاف ليكويد روتريتو 0.7 مم أحمر', 10, 
  '{"Color": "Red", "Ink Type": "Liquid Ink", "Model": "Liquid Ball", "Tip Size": "0.7"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'ROT-LB-0p7MM-BLK', 'Rotring Liquid Ball Pen 0.7mm Black', 'قلم جاف ليكويد روتريتو 0.7 مم أسود', 22, 
  '{"Color": "Black", "Ink Type": "Liquid Ink", "Model": "Liquid Ball", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PENSAN'),
  'PEN-MYT-0p7MM-BLU', 'Pentel My-Tech Pen 0.7mm Blue', 'قلم جاف بينتل ماي تيك 0.7 مم أزرق', 12, 
  '{"Color": "Blue", "Ink Type": "Semi-Gel", "Model": "My-Tech", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PENSAN'),
  'PEN-MYT-0p7MM-GRN', 'Pentel My-Tech Pen 0.7mm Green', 'قلم جاف بينتل ماي تيك 0.7 مم أخضر', 12, 
  '{"Color": "Green", "Ink Type": "Semi-Gel", "Model": "My-Tech-2240", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PENSAN'),
  'PEN-TRB-1p0MM-BLU', 'Pentel Triball Pen 1.0mm Blue', 'قلم جاف بينتل ترايبال 1.0 مم أزرق', 11, 
  '{"Color": "Blue", "Ink Type": "Oil-based", "Model": "TRIBALL", "Shape": "Triangular", "Tip Size": "1.0mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-FRM-1P0MM-BLU', 'Prima Forma Ball Pen 1.0mm Blue', 'قلم جاف بريما فورما 1.0 مم أزرق', 16, 
  '{"Color": "Blue", "Ink Type": "Oil-based", "Model": "Forma", "Tip Size": "1.0mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-SLO-0.7MM-NEEDLE', 'Prima Solo Needle Point Pen 0.7mm Blue', 'قلم جاف بريما سولو سن إبرة 0.7 مم أزرق', 20, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Solo", "Tip Size": "0.7mm", "Tip Type": "Needle Point"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-LNO-1.0MM-BLU', 'Prima Lino Ball Pen 1.0mm Blue', 'قلم جاف بريما لينو 1.0 مم أزرق', 8, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Lino", "Tip Size": "1.0mm", "Tip Type": "Bullet"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-NVA-0.7MM-BLK', 'Prima Nova Ball Pen 0.7mm Black', 'قلم جاف بريما نوفا 0.7 مم أسود', 10, 
  '{"Color": "Black", "Ink Type": "Ballpoint", "Model": "NOVA", "Tip Size": "0.7mm", "Tip Type": "Bullet"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-FRS-0p.7MM-BLU', 'Prima Forsa Ball Pen 0.7mm Blue', 'قلم جاف بريما فورسا 0.7 مم أزرق', 9, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "FORSA", "Tip Size": "0.7mm", "Tip Type": "Bullet"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-MGR-0p7MM-BLU', 'Prima Magro Ball Pen 0.7mm Blue', 'قلم جاف بريما ماجرو 0.7 مم أزرق', 9, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "MAGRO", "Tip Size": "0.7mm", "Tip Type": "TC Ball"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-TRT-0p7MM-BLU', 'Prima Tri-touch Ball Pen 0.7mm Blue', 'قلم جاف بريما تراي تاتش 0.7 مم أزرق', 11, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Tri-touch", "Shape": "Triangular", "Tip Size": "0.7mm", "Tip Type": "Bullet"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-GNT-0p7MM-BLU', 'Prima Genta Ball Pen 0.7mm Blue', 'قلم جاف بريما جينتا 0.7 مم أزرق', 5, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Genta", "Tip Size": "0.7mm", "Tip Type": "Bullet"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'ROT-ESF-1p0MM-RED', 'Rotring Easy Flow Liquid Pen 1.0mm Red', 'قلم إيزي فلو روتريتو 1.0 مم أحمر', 21, 
  '{"Color": "Red", "Ink Type": "Liquid Ink", "Model": "Easy Flow", "Tip Size": "1.0mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-25-0p7MM-BLU', 'Prima 25 Ball Pen 0.7mm Blue', 'قلم جاف بريما 25-0.7 مم أزرق', 7, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "25", "Tip Size": "0.7mm", "Tip Type": "Bullet"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'ROT-GPL-0p7MM-BLU', 'Rotring G-Plus Liquid Pen 0.7mm Blue', 'قلم جاف جي بلس روتريتو 0.7 مم أزرق', 12, 
  '{"Color": "Blue", "Ink Type": "Liquid Ink", "Model": "G-PLUS", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-GLS-0p7MM-BLU', 'Rotring Glase Liquid Pen 0.7mm Blue', 'قلم جاف جليز روتريتو 0.7 مم أزرق', 3, 
  '{"Color": "Blue", "Ink Type": "Liquid Ink", "Model": "GLASE", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Pen'),
  (SELECT id FROM public.brands WHERE name = 'MAPED'),
  'MPD-MPC-0p5MM', 'Maped Mechanical Pencil 0.5mm', 'قلم رصاص سنون مابيد 0.5 مم', 2, 
  '{"Lead Size": "0.5", "Model": "ET11945007"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Ruler'),
  (SELECT id FROM public.brands WHERE name = 'DOMS'),
  'DMS-RUL-MTC-20CM', 'Doms Metric Ruler 20cm Plastic', 'مسطرة دومس 20 سم بلاستيك', 10, 
  '{"Material": "Plastic", "Width": "30mm", "length": "20cm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Modeling Clay'),
  (SELECT id FROM public.brands WHERE name = 'DU HU'),
  'DHU-HLT-YEL', 'Dahle Highlighter Yellow Chisel', 'قلم هاي لايتر داهلي أصفر مشطوف', 10, 
  '{"Color": "Fluorescent Yellow", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Permanent Marker'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-WBM-BLU', 'Rotring Whiteboard Marker Blue Board 501', 'قلم سبورة روتريتو أزرق 501', 12, 
  '{"Color": "Blue", "Ink Type": "Dry Wipe", "Model": "Board 501", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Permanent Marker'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-WBM-BLK', 'Rotring Whiteboard Marker Black Board 501', 'قلم سبورة روتريتو أسود 501', 12, 
  '{"Color": "Black", "Ink Type": "Dry Wipe", "Model": "Board 501", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Permanent Marker'),
  (SELECT id FROM public.brands WHERE name = 'ROTO'),
  'RTO-WBM-RED', 'Rotring Whiteboard Marker Red Board 501', 'قلم سبورة روتريتو أحمر 501', 12, 
  '{"Color": "Red", "Ink Type": "Dry Wipe", "Model": "Board 501", "Tip Style": "Chisel"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-ORO-0p7MM-BLU', 'Prima Oro Ball Pen 0.7mm Blue', 'قلم جاف بريما أورو 0.7 مم أزرق', 7, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Oro", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-FNR-0p7MM-BLU', 'Prima Fancy & Rosa Ball Pen 0.7mm Blue', 'قلم جاف بريما فانسي وروسا 0.7 مم أزرق', 12, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Fancy&Rosa", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-BRN-0p7MM-BLU', 'Prima Bronzo Ball Pen 0.7mm Blue', 'قلم جاف بريما برونزو 0.7 مم أزرق', 10, 
  '{"Color": "Blue", "Ink Type": "Ballpoint", "Model": "Bronzo", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-JEL-0p5MM-BLU', 'Prima Gely Fine Point Pen 0.5mm Blue', 'قلم جيل بريما جيلي 0.5 مم أزرق', 20, 
  '{"Color": "Blue", "Ink Type": "Fine point", "Model": "Gely", "Tip Size": "0.5mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-JEL-0p5MM-BLK', 'Prima Gely Fine Point Pen 0.5mm Black', 'قلم جيل بريما جيلي 0.5 مم أسود', 10, 
  '{"Color": "Black", "Ink Type": "Fine point", "Model": "Gely", "Tip Size": "0.5mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-JEL-0p5MM-RED', 'Prima Gely Fine Point Pen 0.5mm Red', 'قلم جيل بريما جيلي 0.5 مم أحمر', 10, 
  '{"Color": "Red", "Ink Type": "Fine point", "Model": "Gely", "Tip Size": "0.5mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Invoice Book'),
  (SELECT id FROM public.brands WHERE name = 'PRIMA'),
  'PRM-FNO-0p7MM-BLK', 'Prima Fino Ball Pen 0.7mm Black', 'قلم جاف بريما فينو 0.7 مم أسود', 10, 
  '{"Color": "Black", "Ink Type": "Ballpoint", "Model": "Fino", "Tip Size": "0.7mm"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name = 'POWER'),
  'PWR-PCL-2B', 'Power Maden Pencil 2B with Eraser', 'قلم رصاص باور مدن 2B بالمسطرة والاستيكة', 36, 
  '{"Degree": "2B", "Eraser": "true", "Model": "Maden", "Shape": "Cylindrical"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name = 'GAZELLE'),
  'GZL-ENV-DL-WLT-80G-WHT', 'Ghazala DL Envelope Gummed 80g White', 'ظرف غزالة DL صمغ 80 جرام أبيض', 150, 
  '{"Closure Type": "Gummed", "Color": "White", "Dimensions": "110*220mm", "Paper Weight": "80g", "Size": "DL"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name = 'ELNOUR'),
  'GZL-ENV-DL-SLF-80G-WHT', 'Ghazala DL Envelope Self-Seal 80g White', 'ظرف غزالة DL ذاتي اللصق 80 جرام أبيض', 40, 
  '{"Closure Type": "Self-Seal", "Color": "White", "Dimensions": "110*220mm", "Paper Weight": "80g", "Size": "DL"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name = 'GAZELLE'),
  'GZL-ENV-C5-SLF-100G-WHT', 'Ghazala C5 Envelope Self-Seal 100g White', 'ظرف غزالة C5 ذاتي اللصق 100 جرام أبيض', 150, 
  '{"Closure Type": "Self-Seal", "Color": "White", "Dimensions": "162*229mm", "Paper Weight": "100g", "Size": "C5"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name = 'GAZELLE'),
  'GZL-ENV-C4-SLF-100G-WHT-COPY', 'Ghazala C4 Envelope Self-Seal 100g White', 'ظرف غزالة C4 ذاتي اللصق 100 جرام أبيض', 50, 
  '{"Closure Type": "Self-Seal", "Color": "White", "Dimensions": "229*324mm", "Paper Weight": "100g", "Size": "C4"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Plastic Envelope with Button'),
  (SELECT id FROM public.brands WHERE name = 'AMIRA FACTORY ALEX'),
  'AMR-ENV-DL-AIR-GUM-60G-WHT', 'Amir DL Envelope Air Gummed 80g White', 'ظرف طيران الأمير DL صمغ 80 جرام أبيض', 50, 
  '{"Closure Type": "Gummed", "Color": "White", "Dimensions": "110*220mm", "Paper Weight": "80g", "Size": "DL"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-BTN-FC-209-10', 'Everygreen Plastic Envelope Button FC 209-10', 'حافظة كبسولة بلاستيك افيفرجرين FC 209-10', 25, 
  '{"Color": "Multi-color", "Dimensions": "25*35cm", "Material": "Plastic", "Szie": "FC (Foolscap)", "model": "209-10"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-BTN-FC-209-14', 'Everygreen Plastic Envelope Button FC 209-14', 'حافظة كبسولة بلاستيك افيفرجرين FC 209-14', 90, 
  '{"Color": "Multi-color", "Dimensions": "25*35cm", "Material": "Plastic", "Szie": "FC (Foolscap)", "model": "209-14"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-BTN-FC-209-1-18C', 'Everygreen Plastic Envelope Button FC 209-1-18C', 'حافظة كبسولة بلاستيك افيفرجرين FC 209-1-18C', 20, 
  '{"Color": "Multi-color", "Dimensions": "25*35cm", "Material": "Plastic", "Szie": "FC (Foolscap)", "model": "209-1-18C"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Stamp Pad Refill Ink'),
  (SELECT id FROM public.brands WHERE name = 'ELSAFA'),
  'SAF-DWG-17X24-70G-18S', 'Safa Sketchbook 17x24cm 18 Sheets 70g', 'كراس رسم الصفا 17*24 سم 18 ورقة 70 جرام', 12, 
  '{"Color": "White", "Dimensions": "17*24cm", "Paper Weight": "70g", "Sheet Count": "18"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Stamp Pad Refill Ink'),
  (SELECT id FROM public.brands WHERE name = 'GHARIB'),
  'GHR-DWG-17X24-60G-16P', 'El-Gharbawi Sketchbook 17x24cm 16 Sheets 60g', 'كراس رسم الغرباوي 17*24 سم 16 ورقة 60 جرام', 40, 
  '{"Color": "White", "Dimensions": "17*24cm", "Paper Weight": "60g", "Sheet Count": "16"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Glue stick'),
  (SELECT id FROM public.brands WHERE name = 'PASCO'),
  'PAS-GLU-08G-COPY', 'Pasco Glue Stick 8g (Duplicate)', 'جلو ستيك باسك 8 جرام (مكرر)', 29, 
  '{"Material": "PVP Solid Glue", "Uses": "Paper", "Weight": "8g"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Mechanical Pencil'),
  (SELECT id FROM public.brands WHERE name = 'FABER CASTLE'),
  'FAB-2122-2B-COPY', 'Faber-Castell 2122 2B Pencil (Duplicate)', 'قلم رصاص فابر كاستل 2122 2B (مكرر)', 72, 
  '{"Degree": "2B", "Eraser": "false", "Model": "2122", "Shape": "Hexagonal"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Zipper Document Wallet'),
  (SELECT id FROM public.brands WHERE name = 'EVERGREEN'),
  'EVG-BTN-FC-209-1-18C-COPY', 'Everygreen Plastic Envelope FC (Duplicate)', 'حافظة كبسولة بلاستيك افيفرجرين (مكرر)', 20, 
  '{"Color": "Multi-color", "Dimensions": "25*35cm", "Material": "Plastic", "Szie": "FC (Foolscap)", "model": "209-1-18C"}'::jsonb
),
(
  (SELECT id FROM public.products WHERE name = 'Whiteboard Marker'),
  (SELECT id FROM public.brands WHERE name = 'FABER CASTLE'),
  'FAR-LS-BLU', 'Faris Registration Folder Blue', 'ملف تقديم الفارس أزرق', 180, 
  '{"Color": "Blue", "Dimensions": "32.5*22.5", "Paper Weight": "70g"}'::jsonb
)
ON CONFLICT ((lower(sku::text))) DO NOTHING;