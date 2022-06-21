#pragma once

#include "../macros.h"

namespace Alkahest
{
    // Markers represent a point within memory
    typedef std::uint32_t Marker;

    namespace Memory
    {
        struct chunk
        {
            void *start;
            std::uint32_t size;
            void *end;
        };

        inline std::size_t align(std::uint32_t n) { return (n + sizeof(std::size_t) - 1) & ~(sizeof(std::size_t) - 1); };
    }

    class API HeapAllocator
    {
    public:
        HeapAllocator(std::uint32_t size);
        ~HeapAllocator();
    public:
        void *alloc(std::uint32_t size);
        void free(void *obj);
    private:
        void *findAvailableSlot(std::uint32_t size);
    private:
        void *m_heap;
        std::uint32_t m_size;
        std::map<void*,Memory::chunk> m_allocations;
    };

    /**
     * Used to allocate chunks of memory.
     */
    class API StackAllocator
    {
    public:
        /** Allocates a contiguous section of memory.
         *
         * @param size The number of bytes to allocate for this stack.
         */
        StackAllocator(std::uint32_t size);
        ~StackAllocator();
    public:
        /** Returns a chunk of memory from the top of the stack.
         *
         * @param size The number of bytes to allocate from the stack.
         */
        void *alloc(std::uint32_t size);

        /** Clears the stack and resets the top marker. */
        void clear();

        /** Returns the position of the top of the stack. */
        Marker getMarker();

        /** Clears the stack down to a previous marker. All data
         * below the marker is preserved in the stack.
         *
         * @param m A previous marker that is below the current top of the stack.
         */
        void freeToMarker(Marker m);
    private:
        Marker m_topMarker;
        void *m_stack;
        std::uint32_t m_size;
    };

    template<typename T>
    class API PoolAllocator
    {
    public:
        /** Allocates a pool of memory that can contain `size` elements of type T.
         *
         * @param size The maximum number of objects of type T to be held by
         *             the pool.
         */
        PoolAllocator(std::uint32_t size) : m_size(Memory::align(size))
        {
            AKST_ENG_ASSERT(size > 0, "Allocated size must be greater that zero!");

            // Allocate memory pool
            m_pool = std::malloc(Memory::align(size * sizeof(T)));

            // Load queue with available indices
            for (std::uint32_t i = 0; i < size; i++)
            {
                m_available.push(reinterpret_cast<void*>(reinterpret_cast<char*>(m_pool) \
                    + (i * sizeof(T))));
            }
        };

        ~PoolAllocator()
        {
            std::free(m_pool);
        };
    public:
        /** Returns a pointer to the next available slot in the memory pool. */
        void *alloc()
        {
            AKST_ENG_ASSERT(m_available.size() > 0, "Memory pool is full!");

            void *chunk = m_available.front();
            m_available.pop();
            return chunk;
        };

        /** Returns an allocated slot to the pool for reallocation at
         * a later time. The data at the pointer address is not cleared as
         * it is expected to be overwritten by new data after being allocated.
         *
         * @param t A pointer to the address that should be returned to the pool.
         */
        void dealloc(T* t)
        {
            AKST_ENG_ASSERT( \
                reinterpret_cast<void*>(t) >= m_pool \
                && reinterpret_cast<void*>(t) < \
                reinterpret_cast<void*>(reinterpret_cast<char*>(m_pool) + (m_size * sizeof(T))), \
                "Address to deallocate is outside memory pool!");

            m_available.push(reinterpret_cast<void*>(t));
        };
    private:
        std::queue<void*> m_available;
        void *m_pool;
        std::uint32_t m_size;
    };
}
