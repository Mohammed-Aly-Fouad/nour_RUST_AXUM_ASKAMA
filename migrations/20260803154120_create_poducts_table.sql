-- ========================================================
-- 3. Products Table
-- ========================================================
CREATE TABLE IF NOT EXISTS public.products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    category_id INT NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Explicit Foreign Key Constraint
    CONSTRAINT fk_products_category
        FOREIGN KEY (category_id) 
        REFERENCES public.categories (id) 
        ON DELETE RESTRICT 
        ON UPDATE CASCADE
);

-- Foreign key lookup index
CREATE INDEX IF NOT EXISTS idx_products_category_id 
    ON public.products (category_id);

-- Case-Insensitive Unique Indexes
-- Enforces uniqueness while treating 'Product A', 'product a', and 'PRODUCT A' as identical
CREATE UNIQUE INDEX IF NOT EXISTS idx_products_unique_name_lower 
    ON public.products (LOWER(name));

CREATE UNIQUE INDEX IF NOT EXISTS idx_products_unique_name_ar_lower 
    ON public.products (LOWER(name_ar));

-- Updated-at trigger
DROP TRIGGER IF EXISTS update_products_modtime ON public.products;
CREATE TRIGGER update_products_modtime
    BEFORE UPDATE ON public.products
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();