-- ========================================================
-- 1. إنشاء دالة تحديث الحقل updated_at تلقائيًا (إن لم تكن موجودة)
-- ========================================================
CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 2. إنشاء جدول Brands والقيود الخاصة به
-- ========================================================
CREATE TABLE IF NOT EXISTS public.brands (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- القيود الفريدة لمنع تكرار الأسماء
    CONSTRAINT unique_brand_name UNIQUE (name),
    CONSTRAINT unique_brand_name_ar UNIQUE (name_ar)
);

-- ========================================================
-- 3. ربط الـ Trigger بالجدول لتشغيل الدالة عند كل UPDATE
-- ========================================================
DROP TRIGGER IF EXISTS update_brands_modtime ON public.brands;

CREATE TRIGGER update_brands_modtime
    BEFORE UPDATE ON public.brands
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();