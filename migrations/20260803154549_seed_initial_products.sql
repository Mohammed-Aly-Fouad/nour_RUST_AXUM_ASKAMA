INSERT INTO public.products (category_id, name, name_ar) VALUES 
-- 212: Adhesives & Glues
((SELECT id FROM public.categories WHERE name = 'Adhesives & Glues'), 'Glue stick', 'جلو ستيك'),
((SELECT id FROM public.categories WHERE name = 'Adhesives & Glues'), 'Sellotape', 'شريط لاصق سلوتيب'),
((SELECT id FROM public.categories WHERE name = 'Adhesives & Glues'), 'Glue Gun Stick', 'غيار مسدس شمع'),

-- 603: Art Supplies, Sketchbooks & Colors
((SELECT id FROM public.categories WHERE name = 'Art Supplies, Sketchbooks & Colors'), 'Colored Pencil', 'ألوان خشب'),
((SELECT id FROM public.categories WHERE name = 'Art Supplies, Sketchbooks & Colors'), 'Sketchbook', 'كراس رسم'),

-- 501: Calculators, Storage & Tech Accessories
((SELECT id FROM public.categories WHERE name = 'Calculators, Storage & Tech Accessories'), 'Calculator', 'ألة حاسبة'),
((SELECT id FROM public.categories WHERE name = 'Calculators, Storage & Tech Accessories'), 'Dry Cell Battery', 'بطارية جافة'),
((SELECT id FROM public.categories WHERE name = 'Calculators, Storage & Tech Accessories'), 'USB Flash Drive', 'فلاش ميموري'),

-- 203: Erasers & Correction
((SELECT id FROM public.categories WHERE name = 'Erasers & Correction'), 'Correction Pen', 'قلم كوريكتور'),
((SELECT id FROM public.categories WHERE name = 'Erasers & Correction'), 'Eraser', 'استيكة'),

-- 401: Files, Folders & Envelopes
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Pocket Display Book', 'هولدر جيوب متعددة'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'U-Shape Sheet Protector', 'حافظة حرف يو'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'L-Shape Sheet Protector', 'حافظة حرف إل'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Envelope', 'ظرف'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Plastic Envelope with Button', 'حافظة كبسولة'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Zipper Document Wallet', 'حافظة سوستة'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Loose-Leaf Ring Binder', 'كلاسير حلقات'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Slide Binder', 'دوسيه مسطرة'),
((SELECT id FROM public.categories WHERE name = 'Files, Folders & Envelopes'), 'Report Cover', 'دوسيه تقارير'),

-- 602: Geometry & Measuring Tools
((SELECT id FROM public.categories WHERE name = 'Geometry & Measuring Tools'), 'Stencil Ruler', 'مسطرة أشكال'),
((SELECT id FROM public.categories WHERE name = 'Geometry & Measuring Tools'), 'Protractor', 'منقلة'),
((SELECT id FROM public.categories WHERE name = 'Geometry & Measuring Tools'), 'Ruler', 'مسطرة'),
((SELECT id FROM public.categories WHERE name = 'Geometry & Measuring Tools'), 'Compass', 'برجل'),

-- 802: Party & Birthday Supplies
((SELECT id FROM public.categories WHERE name = 'Party & Birthday Supplies'), 'Wrapping Ribbon', 'شريط لف الهدايا'),
((SELECT id FROM public.categories WHERE name = 'Party & Birthday Supplies'), 'Balloon', 'بالون'),

-- 306: Legal Contracts & Ready Forms
((SELECT id FROM public.categories WHERE name = 'Legal Contracts & Ready Forms'), 'Legal Contract', 'عقود'),
((SELECT id FROM public.categories WHERE name = 'Legal Contracts & Ready Forms'), 'Receipt', 'إيصالات'),
((SELECT id FROM public.categories WHERE name = 'Legal Contracts & Ready Forms'), 'registration folder', 'ملف تقديم'),
((SELECT id FROM public.categories WHERE name = 'Legal Contracts & Ready Forms'), 'Lease agreement', 'عقد إيجار'),

-- 205: Markers & Highlighters
((SELECT id FROM public.categories WHERE name = 'Markers & Highlighters'), 'Whiteboard Marker', 'قلم سبورة'),
((SELECT id FROM public.categories WHERE name = 'Markers & Highlighters'), 'Permanent Marker', 'قلم ماركر دائم'),
((SELECT id FROM public.categories WHERE name = 'Markers & Highlighters'), 'Highlighter', 'قلم هاي لايتر'),

-- 801: Modeling Dough & Crafts
((SELECT id FROM public.categories WHERE name = 'Modeling Dough & Crafts'), 'Modeling Clay', 'صلصال'),

-- 101: Notebooks, Notepads & Journals
((SELECT id FROM public.categories WHERE name = 'Notebooks, Notepads & Journals'), 'Pocket Notebook', 'نوت بوك صغيرة'),
((SELECT id FROM public.categories WHERE name = 'Notebooks, Notepads & Journals'), 'Exercise Book', 'كراسة'),
((SELECT id FROM public.categories WHERE name = 'Notebooks, Notepads & Journals'), 'Notebook', 'كشكول'),
((SELECT id FROM public.categories WHERE name = 'Notebooks, Notepads & Journals'), 'Ruled Paper', 'ورق مسطر'),

-- 701: Pencil Cases & School Accessories
((SELECT id FROM public.categories WHERE name = 'Pencil Cases & School Accessories'), 'Pencil Case', 'مقلمة'),

-- 201: Pencils & Lead Refills
((SELECT id FROM public.categories WHERE name = 'Pencils & Lead Refills'), 'Pencil Sharpener', 'براية'),
((SELECT id FROM public.categories WHERE name = 'Pencils & Lead Refills'), 'Graphite Pencil', 'قلم رصاص'),
((SELECT id FROM public.categories WHERE name = 'Pencils & Lead Refills'), 'Mechanical Pencil', 'قلم رصاص سنون'),

-- 206: Pens & Refills
((SELECT id FROM public.categories WHERE name = 'Pens & Refills'), 'Pen', 'قلم جاف'),

-- 304: Pricing, Invoicing & Thermal Rolls
((SELECT id FROM public.categories WHERE name = 'Pricing, Invoicing & Thermal Rolls'), 'Invoice Book', 'دفتر فواتير'),
((SELECT id FROM public.categories WHERE name = 'Pricing, Invoicing & Thermal Rolls'), 'Price Labels', 'ملصقات الأسعار'),
((SELECT id FROM public.categories WHERE name = 'Pricing, Invoicing & Thermal Rolls'), 'Thermal Paper Roll', 'بكر حراري'),
((SELECT id FROM public.categories WHERE name = 'Pricing, Invoicing & Thermal Rolls'), 'Custom Invoice Book', 'دفتر فواتير مخصص'),

-- 305: Staplers, Clips, Rubber Bands & Desk Accessories
((SELECT id FROM public.categories WHERE name = 'Staplers, Clips, Rubber Bands & Desk Accessories'), 'Rubber Bands', 'أساتك'),
((SELECT id FROM public.categories WHERE name = 'Staplers, Clips, Rubber Bands & Desk Accessories'), 'Bulldog Clip', 'مشبك ورق'),
((SELECT id FROM public.categories WHERE name = 'Staplers, Clips, Rubber Bands & Desk Accessories'), 'Clipboard', 'بلانشيطة'),
((SELECT id FROM public.categories WHERE name = 'Staplers, Clips, Rubber Bands & Desk Accessories'), 'Sticky Note', 'سيتكي نوت'),

-- 302: Stamps & Inks
((SELECT id FROM public.categories WHERE name = 'Stamps & Inks'), 'Stamp Pad Refill Ink', 'حبر ختامة')

ON CONFLICT ((lower(name::text))) DO NOTHING;