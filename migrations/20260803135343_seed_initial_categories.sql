-- 1. إدخال الفئات الأساسية (Main Categories - Parents)
INSERT INTO public.categories (id, name, name_ar, parent_id, notes) VALUES 
(1, 'Notebooks', 'الدفاتر', NULL, NULL),
(2, 'Writing & Correction', 'أدوات الكتابة والتصحيح', NULL, NULL),
(3, 'Office Supplies', 'مستلزمات مكتبية', NULL, NULL),
(4, 'Filing & Organization', 'حفظ وتنظيم الملفات', NULL, NULL),
(5, 'Technology & Electronics', 'تكنولوجيا وإلكترونيات', NULL, NULL),
(6, 'Drawing and measuring tools', 'أدوات الرسم والقياس', NULL, NULL),
(7, 'School & College Supplies', 'الأدوات المدرسية والجامعية', NULL, NULL),
(8, 'Toys, Games, Gifts & Party Supplies', 'الألعاب والهدايا ومستلزمات الحفلات', NULL, 'Main section for children toys, educational games, modeling dough, crafts, party decorations, and gift wrapping supplies'),

-- 2. إدخال الفئات الفرعية (Sub-Categories - Children)
(101, 'Notebooks, Notepads & Journals', 'الكشاكيل والدفاتر والنوت بوك', 1, 'Includes wirebound notebooks, pocket journals, composition books, notepad sets, and loose foolscap/exam paper'),
(201, 'Pencils & Lead Refills', 'أقلام رصاص وسنون ', 2, 'Includes wooden pencils, mechanical pencils, pencil sharpeners, and graphite lead refills'),
(203, 'Erasers & Correction', 'أدوات المحو والتصحيح', 2, 'Includes erasers, correction tapes, and correction fluids'),
(205, 'Markers & Highlighters', 'أقلام ماركر وتحديد', 2, 'Includes permanent markers, whiteboard markers, and text highlighters'),
(206, 'Pens & Refills', 'أقلام جاف وجيل وحبر', 2, 'Includes ballpoint pens, gel pens, rollerball pens, technical drawing pens, and ink refills'),
(212, 'Adhesives & Glues', 'مواد وأدوات اللصق', 3, 'Includes glue sticks, hot melt glue, liquid glue, and tapes'),
(302, 'Stamps & Inks', 'أختام وأحبار', 3, 'Includes stamp pads, refill inks, and custom stamps'),
(304, 'Pricing, Invoicing & Thermal Rolls', 'أدوات التسعير ودفاتر الفواتير والبكر الحراري', 3, 'Includes pricing guns, labels, thermal rolls, invoice books, receipt vouchers, and bill books'),
(305, 'Staplers, Clips, Rubber Bands & Desk Accessories', 'دباسات ومشابك وأساتك ومستلزمات مكتبية', 3, 'Includes staplers, staples, bulldog clips, paper clips, rubber bands of various sizes, punchers, clipboards, and sticky notes'),
(306, 'Legal Contracts & Ready Forms', 'العقود القانونية والنماذج الجاهزة', 3, 'Includes ready-made legal contracts, lease agreements, sales contracts, and generic business forms'),
(401, 'Files, Folders & Envelopes', 'ملفات ودوسيهات وأظرف ورقية', 4, 'Includes display books, clear sleeves, report covers, flat folders, and all types of mailing and shipping envelopes'),
(501, 'Calculators, Storage & Tech Accessories', 'الآلات الحاسبة ووسائط التخزين وإكسسوارات التكنولوجيا', 5, 'Includes scientific and desktop calculators, USB flash drives, memory cards, and tech accessories'),
(601, 'Gift Wrapping Supplies', 'مستلزمات وتغليف الهدايا', 8, 'Includes gift ribbons, cellophane sheets, wrapping accessories, and party-related gift packaging'),
(602, 'Geometry & Measuring Tools', 'أدوات القياس والهندسة', 6, 'Includes rulers, protractors, compasses, set squares, and complete geometry boxes'),
(603, 'Art Supplies, Sketchbooks & Colors', 'أدوات الرسم والاسكتشات والألوان', 6, 'Includes sketchbooks, art papers, colored pencils, crayons, felt-tip markers, watercolors, and drawing accessories'),
(701, 'Pencil Cases & School Accessories', 'المقالم والمستلزمات المدرسية', 7, 'Includes fabric pencil cases, hardtop organizers, multi-layer pouches, and basic student accessories'),
(702, 'Educational Books & Study Guides', 'الكتب الخارجية والمذكرات التعليمية', 7, 'Includes primary, preparatory, and high school educational textbooks, revision guides, and teacher editions'),
(801, 'Modeling Dough & Crafts', 'الصلصال والأنشطة اليدوية للاطفال', 8, 'Includes Foam clay, slime, and playdough sets'),
(802, 'Party & Birthday Supplies', 'مستلزمات الحفلات وأعياد الميلاد', 8, 'Includes balloons, banners, party poppers, candles, and birthday decorations')
ON CONFLICT (id) DO NOTHING;

-- 3. تعديل الـ Sequence ليكون جاهزاً للـ IDs القادمة (أعلى رقم حالياً هو 802)
SELECT setval(pg_get_serial_sequence('public.categories', 'id'), COALESCE(MAX(id), 1)) FROM public.categories;