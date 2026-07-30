-- 1. Drop existing table if starting fresh (optional)
DROP TABLE IF EXISTS brands CASCADE;

-- 2. Create the table schema
CREATE TABLE brands (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    name_ar VARCHAR(255) UNIQUE NOT NULL,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 3. Create the automatic updated_at trigger function
CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 4. Attach the trigger
CREATE TRIGGER update_brands_modtime
    BEFORE UPDATE ON brands
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

-- 5. Seed Data
"INSERT INTO brands (id, name, name_ar, notes) VALUES (1, 'FABER CASTLE', 'فابر كاستل', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (2, 'DELI', 'ديلي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (3, 'MADEN', 'مادن', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (4, 'HONG_WEI', 'هونج وي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (5, 'FARAG', 'فراج', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (6, 'EVERGREEN', 'إيفرجرين', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (7, 'FLOWER', 'فلاور', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (8, 'DL_DINGLI', 'دينجلي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (9, 'MIDGO', 'ميدجو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (10, 'PASCO', 'باسكو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (11, 'ELUCKY_EVERLUCKY', 'إيفر لاكي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (12, 'SOFI PLAST', 'سوفي بلاست', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (13, 'SEMA', 'سيما', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (14, 'BRAVO', 'برافو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (15, 'SASCO', 'ساسكو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (16, 'DOMS', 'دومز', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (17, 'ALWARRAK', 'الوراق', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (18, 'MICHAEL', 'مايكل', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (19, 'GAOERJIEFU', 'غاورجيفو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (20, 'MICRO', 'مايكرو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (21, 'QUEEN', 'كوين', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (22, 'PRIMA', 'بريما', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (23, 'ROTO', 'روتو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (24, 'PENSAN', 'بنسان', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (25, 'MAPED', 'مابيد', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (26, 'DU HU', 'دو هو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (27, 'GAZELLE', 'غزال', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (28, 'ELSAFA', 'الصفا', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (29, 'ALADIB', 'الأديب', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (30, 'HOMSON', 'هومسون', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (31, 'ETMAN GROUP', 'عتمان', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (32, 'KANEX', 'كانكس', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (33, 'GOLDEN_TAPE', 'جولدن تيب', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (34, 'KANGARO', 'كانجارو', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (35, 'SELECT', 'سيليكت', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (36, 'RAMSIS', 'رمسيس', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (37, 'AMIRA FACTORY ALEX', 'مصنع أميرة', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (38, 'ELNOUR', 'النور', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (39, 'KINGMAX', 'كينج ماكس', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (40, 'GHARIB', 'أولاد غريب', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (41, 'SENA', 'سنا', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (42, 'STD', 'إس تي دي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (43, 'MAHGOUB', 'محجوب', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (44, 'O & A', 'أو أند إيه', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (45, 'DONG A', 'دونج إيه', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (46, 'ANDLOSYA GROUP', 'أندلسية', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (47, 'KARUIDA', 'كارويدا', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (48, 'CASINE', 'كاسين', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (49, 'PHILIPS', 'فيليبس', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (50, 'GELCY', 'جيلسي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (51, 'AL_AZHARI', 'الأزهري', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (52, 'POWER', 'باور', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (53, 'DIGITAL', 'ديجيتال', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (54, 'CHANYI', 'تشان يي', NULL);"
"INSERT INTO brands (id, name, name_ar, notes) VALUES (58, 'ROTRING', 'روترينج', NULL);"