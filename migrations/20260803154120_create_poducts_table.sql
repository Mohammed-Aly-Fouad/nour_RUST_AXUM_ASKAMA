-- ========================================================
-- 3. Products Table
-- ========================================================
CREATE TABLE IF NOT EXISTS public.products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    name_ar VARCHAR(255) NOT NULL,
    brand_id INT NOT NULL,
    category_id INT NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT products_brand_id_fkey 
        FOREIGN KEY (brand_id) 
        REFERENCES public.brands (id) 
        ON DELETE RESTRICT 
        ON UPDATE CASCADE,

    CONSTRAINT products_category_id_fkey 
        FOREIGN KEY (category_id) 
        REFERENCES public.categories (id) 
        ON DELETE RESTRICT 
        ON UPDATE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_products_brand_id ON public.products (brand_id);
CREATE INDEX IF NOT EXISTS idx_products_category_id ON public.products (category_id);

DROP TRIGGER IF EXISTS update_products_modtime ON public.products;
CREATE TRIGGER update_products_modtime
    BEFORE UPDATE ON public.products
    FOR EACH ROW
    EXECUTE FUNCTION public.update_modified_column();