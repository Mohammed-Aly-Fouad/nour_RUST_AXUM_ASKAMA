-- ========================================================
-- 2. Categories Table
-- ========================================================
CREATE TABLE IF NOT EXISTS public.categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    parent_id INT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Foreign Key Constraint
    CONSTRAINT fk_categories_parent
        FOREIGN KEY (parent_id) 
        REFERENCES public.categories (id) 
        ON DELETE RESTRICT 
        ON UPDATE CASCADE
);

-- Foreign key lookup index
CREATE INDEX IF NOT EXISTS idx_categories_parent_id 
    ON public.categories (parent_id);

-- Case-Insensitive Unique Indexes (Per Parent)
-- Uses NULLS NOT DISTINCT (PostgreSQL 15+) so top-level categories (parent_id IS NULL) are also strictly checked
CREATE UNIQUE INDEX IF NOT EXISTS idx_categories_unique_name_parent_lower 
    ON public.categories (LOWER(name), parent_id) NULLS NOT DISTINCT;

CREATE UNIQUE INDEX IF NOT EXISTS idx_categories_unique_name_ar_parent_lower 
    ON public.categories (LOWER(name_ar), parent_id) NULLS NOT DISTINCT;

-- Enforce 2-Level Depth Limit Function & Trigger
CREATE OR REPLACE FUNCTION public.check_category_depth()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_id IS NOT NULL THEN
        IF EXISTS (
            SELECT 1 FROM public.categories 
            WHERE id = NEW.parent_id AND parent_id IS NOT NULL
        ) THEN
            RAISE EXCEPTION 'Category hierarchy cannot exceed 2 levels (Parent -> Child)';
        END IF;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS enforce_category_depth ON public.categories;
CREATE TRIGGER enforce_category_depth
    BEFORE INSERT OR UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.check_category_depth();

-- Updated-at trigger
DROP TRIGGER IF EXISTS update_categories_modtime ON public.categories;
CREATE TRIGGER update_categories_modtime
    BEFORE UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();