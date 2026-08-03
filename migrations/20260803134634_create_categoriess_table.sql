-- ========================================================
-- 1. Create Categories Table with RESTRICT on Delete
-- ========================================================
CREATE TABLE IF NOT EXISTS public.categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    parent_id INT,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Unique Constraints
    CONSTRAINT unique_english_name UNIQUE (name),
    CONSTRAINT unique_arabic_name UNIQUE (name_ar),
    CONSTRAINT unique_name_per_parent UNIQUE (name, parent_id),

    -- Foreign Key with Safety Safeguards
    CONSTRAINT categories_parent_id_fkey 
        FOREIGN KEY (parent_id) 
        REFERENCES public.categories (id) 
        ON DELETE RESTRICT      -- Protects against accidental deletion of parents with children
        ON UPDATE CASCADE       -- Automatically syncs child records if parent ID changes
);

-- ========================================================
-- 2. Enforce 2-Level Depth Limit (Parent -> Child)
-- ========================================================
CREATE OR REPLACE FUNCTION public.check_category_depth()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.parent_id IS NOT NULL THEN
        -- Prevent creating a 3rd level (Child cannot become a Parent)
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

-- ========================================================
-- 3. Automatic updated_at Timestamp Trigger
-- ========================================================
DROP TRIGGER IF EXISTS update_categories_modtime ON public.categories;

CREATE TRIGGER update_categories_modtime
    BEFORE UPDATE ON public.categories
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();