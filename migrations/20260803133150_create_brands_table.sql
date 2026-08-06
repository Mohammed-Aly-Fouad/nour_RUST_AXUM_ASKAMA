-- ========================================================
-- 0. Helper Functions (Must be defined first)
-- ========================================================
CREATE OR REPLACE FUNCTION public.update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ========================================================
-- 1. Brands Table
-- ========================================================
CREATE TABLE IF NOT EXISTS public.brands (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Case-Insensitive Unique Indexes
-- Enforces uniqueness while treating 'Nike', 'nike', and 'NIKE' as identical
CREATE UNIQUE INDEX IF NOT EXISTS idx_brands_unique_name_lower 
    ON public.brands (LOWER(name));

CREATE UNIQUE INDEX IF NOT EXISTS idx_brands_unique_name_ar_lower 
    ON public.brands (LOWER(name_ar));

-- Updated-at trigger
DROP TRIGGER IF EXISTS update_brands_modtime ON public.brands;
CREATE TRIGGER update_brands_modtime
    BEFORE UPDATE ON public.brands
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();