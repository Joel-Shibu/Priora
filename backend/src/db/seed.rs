use sqlx::PgPool;
use uuid::Uuid;

/// Type alias for a subject entry: (semester_index, code, name, modules)
type SubjectEntry = (usize, &'static str, &'static str, Vec<ModuleEntry>);
/// Type alias for a module entry: (module_index, name, topics)
type ModuleEntry = (i32, &'static str, Vec<&'static str>);

/// Seed initial data if the database is empty (no schemes exist).
pub async fn seed_if_empty(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM schemes")
        .fetch_one(pool)
        .await?;

    if count.0 > 0 {
        tracing::info!("Database already has data, skipping seed");
        return Ok(());
    }

    tracing::info!("Seeding database with KTU 2024 Scheme CSE data...");

    // ── 1. Scheme ──────────────────────────────────────────────────────
    let scheme_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "INSERT INTO schemes (id, name, active)
         VALUES (uuid_generate_v4(), '2024 Scheme', true)
         RETURNING id",
    )
    .fetch_one(pool)
    .await?;

    // ── 2. Branch ──────────────────────────────────────────────────────
    let cse_id = sqlx::query_scalar::<_, uuid::Uuid>(
        "INSERT INTO branches (id, name)
         VALUES (uuid_generate_v4(), 'CSE')
         RETURNING id",
    )
    .fetch_one(pool)
    .await?;

    // ── 3. Semesters S1–S8 ─────────────────────────────────────────────
    let mut semester_ids = Vec::new();
    for sem_num in 1..=8 {
        let sem_id = sqlx::query_scalar::<_, uuid::Uuid>(
            "INSERT INTO semesters (id, scheme_id, branch_id, semester_number)
             VALUES (uuid_generate_v4(), $1, $2, $3)
             RETURNING id",
        )
        .bind(scheme_id)
        .bind(cse_id)
        .bind(sem_num)
        .fetch_one(pool)
        .await?;
        semester_ids.push(sem_id);
    }

    // ── 4. Subjects with modules & topics ─────────────────────────────
    // Each entry: (semester_index, subject_code, subject_name, modules)
    // Module: (module_index, module_name, [topic_names])
    let subjects_data: Vec<SubjectEntry> = vec![
        // ═══════════════════════════════════════════════════════════════
        // S1 — First Year, Semester 1 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            1,
            "GAMAT101",
            "Mathematics for Information Science-1",
            vec![
                (
                    1,
                    "Matrices and Linear Systems",
                    vec![
                        "Rank of a matrix",
                        "System of linear equations",
                        "Eigenvalues and eigenvectors",
                        "Cayley-Hamilton theorem",
                    ],
                ),
                (
                    2,
                    "Vector Spaces",
                    vec![
                        "Vector spaces and subspaces",
                        "Linear independence and basis",
                        "Dimension and rank",
                        "Linear transformations",
                    ],
                ),
                (
                    3,
                    "Differential Calculus of Several Variables",
                    vec![
                        "Partial derivatives",
                        "Total derivative and chain rule",
                        "Maxima and minima of functions",
                        "Lagrange multipliers",
                    ],
                ),
                (
                    4,
                    "Multiple Integrals and Applications",
                    vec![
                        "Double integrals",
                        "Triple integrals",
                        "Change of order of integration",
                        "Applications of multiple integrals",
                    ],
                ),
            ],
        ),
        (
            1,
            "GAPHT121",
            "Physics for Information Science",
            vec![
                (
                    1,
                    "Mechanics and Properties of Matter",
                    vec![
                        "Laws of motion and conservation principles",
                        "Elasticity and Hooke's law",
                        "Bending of beams",
                        "Viscosity and surface tension",
                    ],
                ),
                (
                    2,
                    "Electromagnetic Theory",
                    vec![
                        "Coulomb's law and electric field",
                        "Gauss's law and applications",
                        "Magnetic fields and Biot-Savart law",
                        "Faraday's law and electromagnetic induction",
                    ],
                ),
                (
                    3,
                    "Oscillations, Waves and Optics",
                    vec![
                        "Simple harmonic motion",
                        "Damped and forced oscillations",
                        "Interference and diffraction",
                        "Polarization of light",
                    ],
                ),
                (
                    4,
                    "Quantum Mechanics and Semiconductors",
                    vec![
                        "Photoelectric effect and wave-particle duality",
                        "Schrödinger equation",
                        "Energy bands in solids",
                        "Semiconductor physics and devices",
                    ],
                ),
            ],
        ),
        (
            1,
            "GXCYT122",
            "Chemistry for Information Science",
            vec![
                (
                    1,
                    "Atomic and Molecular Structure",
                    vec![
                        "Atomic orbitals and quantum numbers",
                        "Electronic configuration",
                        "Molecular orbital theory",
                        "Crystal field theory",
                    ],
                ),
                (
                    2,
                    "Chemical Bonding and Spectroscopy",
                    vec![
                        "Ionic and covalent bonding",
                        "Hydrogen bonding and van der Waals forces",
                        "UV-Visible spectroscopy",
                        "IR and NMR spectroscopy basics",
                    ],
                ),
                (
                    3,
                    "Thermodynamics and Reaction Kinetics",
                    vec![
                        "Laws of thermodynamics",
                        "Enthalpy, entropy and free energy",
                        "Rate of reactions and order",
                        "Activation energy and catalysis",
                    ],
                ),
                (
                    4,
                    "Electrochemistry and Corrosion",
                    vec![
                        "Electrochemical cells and Nernst equation",
                        "Conductance and conductivity",
                        "Corrosion and its prevention",
                        "Battery technology basics",
                    ],
                ),
            ],
        ),
        (
            1,
            "GMEST103",
            "Engineering Graphics and Computer Aided Drawing",
            vec![
                (
                    1,
                    "Introduction to Engineering Drawing",
                    vec![
                        "Drawing instruments and their uses",
                        "Sheet layout and title block",
                        "Types of lines and dimensioning",
                        "Geometric constructions",
                    ],
                ),
                (
                    2,
                    "Orthographic and Isometric Projections",
                    vec![
                        "Orthographic projection of points",
                        "Projection of lines and planes",
                        "Projection of solids",
                        "Isometric projection and views",
                    ],
                ),
                (
                    3,
                    "Sectional Views and Development of Surfaces",
                    vec![
                        "Sectioning of solids",
                        "Types of sections",
                        "Development of prisms and pyramids",
                        "Development of cylinders and cones",
                    ],
                ),
                (
                    4,
                    "Computer Aided Drawing Fundamentals",
                    vec![
                        "Introduction to CAD software",
                        "Creating 2D drawings",
                        "Editing and modifying tools",
                        "Dimensioning and plotting",
                    ],
                ),
            ],
        ),
        (
            1,
            "GXEST104",
            "Introduction to Electrical and Electronics Engineering",
            vec![
                (
                    1,
                    "DC Circuits and Network Theorems",
                    vec![
                        "Ohm's law and Kirchhoff's laws",
                        "Mesh and nodal analysis",
                        "Superposition theorem",
                        "Thevenin and Norton theorems",
                    ],
                ),
                (
                    2,
                    "AC Circuits and Transformers",
                    vec![
                        "Sinusoidal steady-state analysis",
                        "Impedance and phasor diagrams",
                        "Power in AC circuits",
                        "Transformer working principle and applications",
                    ],
                ),
                (
                    3,
                    "Semiconductor Diodes and Transistors",
                    vec![
                        "PN junction diode and characteristics",
                        "Rectifiers and power supplies",
                        "Bipolar junction transistor",
                        "Transistor amplifier configurations",
                    ],
                ),
                (
                    4,
                    "Digital Electronics and Logic Gates",
                    vec![
                        "Number systems and binary arithmetic",
                        "Logic gates and truth tables",
                        "Boolean algebra and simplification",
                        "Combinational logic circuits",
                    ],
                ),
            ],
        ),
        (
            1,
            "UCEST105",
            "Algorithmic Thinking with Python",
            vec![
                (
                    1,
                    "Problem Solving and Python Basics",
                    vec![
                        "Algorithm and flowchart fundamentals",
                        "Python installation and environment",
                        "Variables, data types and operators",
                        "Input and output operations",
                    ],
                ),
                (
                    2,
                    "Control Flow and Functions",
                    vec![
                        "Conditional statements (if-elif-else)",
                        "Loops (while and for)",
                        "Function definition and calls",
                        "Recursion and scope of variables",
                    ],
                ),
                (
                    3,
                    "Data Structures in Python",
                    vec![
                        "Lists and list operations",
                        "Tuples and dictionaries",
                        "Sets and their operations",
                        "Strings and string methods",
                    ],
                ),
                (
                    4,
                    "File Handling and Modular Programming",
                    vec![
                        "File operations (read, write, append)",
                        "Exception handling",
                        "Modules and packages",
                        "Introduction to NumPy and Matplotlib",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S2 — First Year, Semester 2 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            2,
            "GAMAT201",
            "Mathematics for Information Science-2",
            vec![
                (
                    1,
                    "Vector Calculus",
                    vec![
                        "Gradient, divergence and curl",
                        "Vector identities",
                        "Line and surface integrals",
                        "Green's theorem and Stokes' theorem",
                    ],
                ),
                (
                    2,
                    "Ordinary Differential Equations",
                    vec![
                        "First order ODE and solution methods",
                        "Second order linear ODE",
                        "Method of undetermined coefficients",
                        "Variation of parameters",
                    ],
                ),
                (
                    3,
                    "Laplace Transforms",
                    vec![
                        "Laplace transform definition and properties",
                        "Inverse Laplace transform",
                        "Solving ODEs using Laplace transforms",
                        "Applications in engineering",
                    ],
                ),
                (
                    4,
                    "Numerical Methods",
                    vec![
                        "Root finding: Newton-Raphson method",
                        "Numerical integration: trapezoidal and Simpson's rules",
                        "Numerical solution of ODEs: Euler and Runge-Kutta",
                        "Interpolation and curve fitting",
                    ],
                ),
            ],
        ),
        (
            2,
            "GXEST203",
            "Foundations of Computing",
            vec![
                (
                    1,
                    "Number Systems and Digital Logic",
                    vec![
                        "Binary, octal and hexadecimal systems",
                        "Boolean algebra and logic gates",
                        "Combinational circuits",
                        "Sequential circuits and flip-flops",
                    ],
                ),
                (
                    2,
                    "Computer Organization Fundamentals",
                    vec![
                        "Basic computer architecture",
                        "CPU components and instruction cycle",
                        "Memory hierarchy",
                        "Input/output organization",
                    ],
                ),
                (
                    3,
                    "Operating System Fundamentals",
                    vec![
                        "Types of operating systems",
                        "Process management",
                        "Memory management basics",
                        "File system concepts",
                    ],
                ),
                (
                    4,
                    "Introduction to Networks and Internet",
                    vec![
                        "Network topologies and types",
                        "OSI and TCP/IP models",
                        "IP addressing and DNS",
                        "World Wide Web and HTTP protocols",
                    ],
                ),
            ],
        ),
        (
            2,
            "GXEST204",
            "Programming in C",
            vec![
                (
                    1,
                    "Basics of C Programming",
                    vec![
                        "Structure of a C program",
                        "Data types and variables",
                        "Operators and expressions",
                        "Control statements (if, switch, loops)",
                    ],
                ),
                (
                    2,
                    "Arrays and Strings",
                    vec![
                        "One-dimensional arrays",
                        "Multi-dimensional arrays",
                        "String handling functions",
                        "Array of strings",
                    ],
                ),
                (
                    3,
                    "Functions and Pointers",
                    vec![
                        "Function definition and prototypes",
                        "Parameter passing mechanisms",
                        "Recursion",
                        "Pointer basics and pointer arithmetic",
                    ],
                ),
                (
                    4,
                    "Structures, Unions and File Handling",
                    vec![
                        "Structure definition and nesting",
                        "Union and enumerated types",
                        "File operations (read, write, append)",
                        "Dynamic memory allocation",
                    ],
                ),
            ],
        ),
        (
            2,
            "PCCST205",
            "Discrete Mathematics",
            vec![
                (
                    1,
                    "Set Theory and Logic",
                    vec![
                        "Sets and set operations",
                        "Propositional logic",
                        "Predicate logic and quantifiers",
                        "Normal forms and logical inference",
                    ],
                ),
                (
                    2,
                    "Relations and Functions",
                    vec![
                        "Properties of relations",
                        "Equivalence relations and partitions",
                        "Partial orders and lattices",
                        "Functions and their properties",
                    ],
                ),
                (
                    3,
                    "Graph Theory",
                    vec![
                        "Graphs and their representations",
                        "Euler and Hamiltonian graphs",
                        "Trees and spanning trees",
                        "Graph coloring and applications",
                    ],
                ),
                (
                    4,
                    "Algebraic Structures",
                    vec![
                        "Groups and subgroups",
                        "Rings and fields",
                        "Boolean algebra",
                        "Applications in computing",
                    ],
                ),
            ],
        ),
        (
            2,
            "UCEST206",
            "Engineering Entrepreneurship and IPR",
            vec![
                (
                    1,
                    "Entrepreneurship and Innovation",
                    vec![
                        "Entrepreneurial mindset and traits",
                        "Types of entrepreneurship",
                        "Innovation and creativity",
                        "Opportunity identification and evaluation",
                    ],
                ),
                (
                    2,
                    "Business Planning and Management",
                    vec![
                        "Business model canvas",
                        "Market analysis and research",
                        "Financial planning and funding sources",
                        "Team building and leadership",
                    ],
                ),
                (
                    3,
                    "Intellectual Property Rights",
                    vec![
                        "Patents and patentability criteria",
                        "Copyrights and trademarks",
                        "Industrial designs and trade secrets",
                        "IP registration process in India",
                    ],
                ),
                (
                    4,
                    "Technology Transfer and Commercialization",
                    vec![
                        "Technology licensing models",
                        "Startup ecosystem in India",
                        "Government schemes and support",
                        "Pitching and investor presentation",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S3 — Second Year, Semester 1 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            3,
            "GAMAT301",
            "Mathematics for Information Science-3",
            vec![
                (
                    1,
                    "Probability and Random Variables",
                    vec![
                        "Probability axioms and theorems",
                        "Random variables and distributions",
                        "Expectation and variance",
                        "Moment generating functions",
                    ],
                ),
                (
                    2,
                    "Probability Distributions",
                    vec![
                        "Binomial and Poisson distributions",
                        "Normal and exponential distributions",
                        "Joint and marginal distributions",
                        "Central limit theorem",
                    ],
                ),
                (
                    3,
                    "Statistical Inference",
                    vec![
                        "Sampling distributions",
                        "Point and interval estimation",
                        "Hypothesis testing",
                        "Chi-square and t-tests",
                    ],
                ),
                (
                    4,
                    "Correlation and Regression",
                    vec![
                        "Correlation coefficient",
                        "Linear regression",
                        "Multiple regression",
                        "Analysis of variance (ANOVA)",
                    ],
                ),
            ],
        ),
        (
            3,
            "PCCST301",
            "Theory of Computation",
            vec![
                (
                    1,
                    "Finite Automata",
                    vec![
                        "Deterministic finite automata",
                        "Non-deterministic finite automata",
                        "NFA to DFA conversion",
                        "Minimization of finite automata",
                    ],
                ),
                (
                    2,
                    "Regular Expressions and Languages",
                    vec![
                        "Regular expressions",
                        "Regular languages and pumping lemma",
                        "Closure properties of regular languages",
                        "Myhill-Nerode theorem",
                    ],
                ),
                (
                    3,
                    "Context-Free Grammars",
                    vec![
                        "Context-free grammars and languages",
                        "Derivation and parse trees",
                        "Pushdown automata",
                        "Pumping lemma for CFL",
                    ],
                ),
                (
                    4,
                    "Turing Machines and Computability",
                    vec![
                        "Turing machine model",
                        "Variants of Turing machines",
                        "Decidable and undecidable problems",
                        "Computability and NP-completeness",
                    ],
                ),
            ],
        ),
        (
            3,
            "PCCST302",
            "Data Structures and Algorithms",
            vec![
                (
                    1,
                    "Linear Data Structures",
                    vec![
                        "Arrays and dynamic arrays",
                        "Linked lists (singly, doubly, circular)",
                        "Stacks and queues",
                        "Applications of stacks and queues",
                    ],
                ),
                (
                    2,
                    "Trees and Hierarchical Structures",
                    vec![
                        "Binary trees and traversals",
                        "Binary search trees",
                        "AVL trees and balancing",
                        "Heap and priority queues",
                    ],
                ),
                (
                    3,
                    "Graph Algorithms",
                    vec![
                        "Graph representations (adjacency matrix and list)",
                        "Depth-first and breadth-first search",
                        "Shortest path algorithms (Dijkstra, Bellman-Ford)",
                        "Minimum spanning trees (Prim's, Kruskal's)",
                    ],
                ),
                (
                    4,
                    "Algorithm Design and Analysis",
                    vec![
                        "Asymptotic analysis (Big O, Omega, Theta)",
                        "Divide and conquer strategy",
                        "Dynamic programming",
                        "Greedy algorithms and their applications",
                    ],
                ),
            ],
        ),
        (
            3,
            "PCCST303",
            "Object Oriented Programming",
            vec![
                (
                    1,
                    "OOP Concepts and Java Fundamentals",
                    vec![
                        "Classes and objects",
                        "Encapsulation and data hiding",
                        "Constructors and destructors",
                        "Method overloading and 'this' keyword",
                    ],
                ),
                (
                    2,
                    "Inheritance and Polymorphism",
                    vec![
                        "Types of inheritance",
                        "Method overriding and dynamic dispatch",
                        "Abstract classes and interfaces",
                        "Polymorphism and its applications",
                    ],
                ),
                (
                    3,
                    "Exception Handling and I/O",
                    vec![
                        "Try-catch-finally blocks",
                        "Custom exceptions",
                        "File I/O streams",
                        "Serialization and deserialization",
                    ],
                ),
                (
                    4,
                    "Generics, Collections and Advanced Topics",
                    vec![
                        "Generic classes and methods",
                        "Collection framework (List, Set, Map)",
                        "Lambda expressions and streams",
                        "Multithreading basics",
                    ],
                ),
            ],
        ),
        (
            3,
            "PCCST304",
            "Digital Electronics and Logic Design",
            vec![
                (
                    1,
                    "Boolean Algebra and Logic Gates",
                    vec![
                        "Boolean algebra theorems",
                        "Logic gates and universal gates",
                        "Karnaugh map simplification",
                        "Quine-McCluskey method",
                    ],
                ),
                (
                    2,
                    "Combinational Logic Circuits",
                    vec![
                        "Adders and subtractors",
                        "Multiplexers and demultiplexers",
                        "Encoders and decoders",
                        "Arithmetic logic unit design",
                    ],
                ),
                (
                    3,
                    "Sequential Logic Circuits",
                    vec![
                        "Latches and flip-flops (SR, JK, D, T)",
                        "Registers and shift registers",
                        "Counters (synchronous and asynchronous)",
                        "Finite state machine design",
                    ],
                ),
                (
                    4,
                    "Memory and Programmable Logic",
                    vec![
                        "RAM and ROM organization",
                        "Programmable logic devices (PLD, PLA, PAL)",
                        "Field programmable gate arrays (FPGA)",
                        "Digital system design using HDL",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S4 — Second Year, Semester 2 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            4,
            "GAMAT401",
            "Mathematics for Information Science-4",
            vec![
                (
                    1,
                    "Complex Analysis",
                    vec![
                        "Complex numbers and functions",
                        "Analytic functions and Cauchy-Riemann equations",
                        "Complex integration and Cauchy's theorem",
                        "Taylor and Laurent series",
                    ],
                ),
                (
                    2,
                    "Partial Differential Equations",
                    vec![
                        "Formation of PDEs",
                        "First order linear PDEs",
                        "Wave and heat equations",
                        "Method of separation of variables",
                    ],
                ),
                (
                    3,
                    "Optimization Techniques",
                    vec![
                        "Linear programming and simplex method",
                        "Duality theory",
                        "Non-linear optimization",
                        "Convex optimization basics",
                    ],
                ),
                (
                    4,
                    "Graph Theory and Combinatorics",
                    vec![
                        "Advanced graph concepts",
                        "Network flow problems",
                        "Combinatorial principles",
                        "Generating functions",
                    ],
                ),
            ],
        ),
        (
            4,
            "PCCST401",
            "Database Management Systems",
            vec![
                (
                    1,
                    "Database Design and ER Model",
                    vec![
                        "Database system architecture",
                        "Entity-relationship model",
                        "Relational model and constraints",
                        "Functional dependencies and normalization",
                    ],
                ),
                (
                    2,
                    "SQL and Query Processing",
                    vec![
                        "DDL, DML and DCL statements",
                        "Joins and subqueries",
                        "Query optimization techniques",
                        "Views and indexes",
                    ],
                ),
                (
                    3,
                    "Transactions and Concurrency Control",
                    vec![
                        "ACID properties",
                        "Transaction states and schedules",
                        "Lock-based protocols",
                        "Deadlock detection and recovery",
                    ],
                ),
                (
                    4,
                    "Storage and Indexing",
                    vec![
                        "File organization and storage structures",
                        "B+ tree indexing",
                        "Hash indexing",
                        "NoSQL databases and big data concepts",
                    ],
                ),
            ],
        ),
        (
            4,
            "PCCST402",
            "Operating Systems",
            vec![
                (
                    1,
                    "Process Management",
                    vec![
                        "Process states and transitions",
                        "CPU scheduling algorithms",
                        "Inter-process communication",
                        "Threads and multithreading models",
                    ],
                ),
                (
                    2,
                    "Memory Management",
                    vec![
                        "Contiguous memory allocation",
                        "Paging and segmentation",
                        "Virtual memory and demand paging",
                        "Page replacement algorithms",
                    ],
                ),
                (
                    3,
                    "File Systems and Storage",
                    vec![
                        "File system interface and operations",
                        "Directory structure and implementation",
                        "File system implementation",
                        "Disk scheduling algorithms",
                    ],
                ),
                (
                    4,
                    "Concurrency and Deadlocks",
                    vec![
                        "Critical section problem",
                        "Semaphores and monitors",
                        "Deadlock characterization",
                        "Deadlock prevention, avoidance and recovery",
                    ],
                ),
            ],
        ),
        (
            4,
            "PCCST403",
            "Computer Organization and Architecture",
            vec![
                (
                    1,
                    "Processor Architecture",
                    vec![
                        "CPU architecture and data path",
                        "Instruction set architecture",
                        "Addressing modes",
                        "RISC vs CISC architectures",
                    ],
                ),
                (
                    2,
                    "Control Unit Design",
                    vec![
                        "Hardwired control unit",
                        "Microprogrammed control unit",
                        "Instruction pipelining",
                        "Hazards and pipeline handling",
                    ],
                ),
                (
                    3,
                    "Memory Hierarchy",
                    vec![
                        "Cache memory organization",
                        "Cache mapping techniques",
                        "Main memory and RAM technologies",
                        "Virtual memory and TLB",
                    ],
                ),
                (
                    4,
                    "I/O Organization and Parallel Processing",
                    vec![
                        "I/O interface and interrupt handling",
                        "DMA and I/O channels",
                        "Parallel processing concepts",
                        "Multi-core architectures",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S5 — Third Year, Semester 1 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            5,
            "PCCST501",
            "Machine Learning",
            vec![
                (
                    1,
                    "Foundations of Machine Learning",
                    vec![
                        "Types of learning (supervised, unsupervised, reinforcement)",
                        "Train-test split and cross-validation",
                        "Bias-variance tradeoff",
                        "Performance metrics and evaluation",
                    ],
                ),
                (
                    2,
                    "Supervised Learning Algorithms",
                    vec![
                        "Linear regression and polynomial regression",
                        "Logistic regression and classification",
                        "Support vector machines",
                        "Decision trees and random forests",
                    ],
                ),
                (
                    3,
                    "Unsupervised Learning",
                    vec![
                        "K-means clustering",
                        "Hierarchical clustering",
                        "Principal component analysis",
                        "Anomaly detection techniques",
                    ],
                ),
                (
                    4,
                    "Neural Networks and Deep Learning",
                    vec![
                        "Neural network architecture and activation functions",
                        "Backpropagation and gradient descent",
                        "Convolutional neural networks basics",
                        "Recurrent neural networks basics",
                    ],
                ),
            ],
        ),
        (
            5,
            "PCCST502",
            "Compiler Design",
            vec![
                (
                    1,
                    "Lexical Analysis",
                    vec![
                        "Tokens, lexemes and patterns",
                        "Regular expressions and finite automata",
                        "Lexical analyzer generator (Lex)",
                        "Symbol table management",
                    ],
                ),
                (
                    2,
                    "Syntax Analysis",
                    vec![
                        "Context-free grammars and derivation",
                        "Top-down parsing (recursive descent, LL)",
                        "Bottom-up parsing (shift-reduce, LR)",
                        "Parser generator (Yacc/Bison)",
                    ],
                ),
                (
                    3,
                    "Semantic Analysis and Intermediate Code",
                    vec![
                        "Syntax-directed definitions",
                        "Type checking and type conversion",
                        "Intermediate code forms (three-address code)",
                        "Symbol table for semantic analysis",
                    ],
                ),
                (
                    4,
                    "Code Generation and Optimization",
                    vec![
                        "Target machine and instruction selection",
                        "Register allocation and assignment",
                        "Peephole optimization",
                        "Machine-independent optimizations",
                    ],
                ),
            ],
        ),
        (
            5,
            "PCCST503",
            "Software Engineering",
            vec![
                (
                    1,
                    "Software Processes and Methodologies",
                    vec![
                        "Waterfall and iterative models",
                        "Agile methodology and Scrum",
                        "Extreme programming and Kanban",
                        "Software process improvement",
                    ],
                ),
                (
                    2,
                    "Requirements Engineering",
                    vec![
                        "Requirements elicitation techniques",
                        "Use case modeling and user stories",
                        "Software requirements specification",
                        "Requirements validation and management",
                    ],
                ),
                (
                    3,
                    "Software Design and Architecture",
                    vec![
                        "Architectural patterns (MVC, layered, microservices)",
                        "Design patterns (Singleton, Factory, Observer)",
                        "UML diagrams and modeling",
                        "Object-oriented design principles",
                    ],
                ),
                (
                    4,
                    "Testing, Maintenance and DevOps",
                    vec![
                        "Unit, integration and system testing",
                        "Test-driven development",
                        "Software maintenance and evolution",
                        "CI/CD pipelines and DevOps practices",
                    ],
                ),
            ],
        ),
        (
            5,
            "PCCST504",
            "Computer Networks",
            vec![
                (
                    1,
                    "Network Fundamentals and Physical Layer",
                    vec![
                        "Network topologies and categories",
                        "OSI and TCP/IP reference models",
                        "Transmission media and signal encoding",
                        "Multiplexing and switching techniques",
                    ],
                ),
                (
                    2,
                    "Data Link Layer",
                    vec![
                        "Error detection and correction codes",
                        "Flow control and sliding window protocols",
                        "MAC protocols (CSMA/CD, CSMA/CA)",
                        "Ethernet and Wi-Fi standards",
                    ],
                ),
                (
                    3,
                    "Network Layer",
                    vec![
                        "IP addressing and subnetting",
                        "Routing algorithms (distance vector, link state)",
                        "IPv4 and IPv6 protocols",
                        "Network Address Translation",
                    ],
                ),
                (
                    4,
                    "Transport and Application Layers",
                    vec![
                        "TCP and UDP protocols",
                        "Congestion control mechanisms",
                        "HTTP, DNS, SMTP protocols",
                        "Network security basics (firewalls, encryption)",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S6 — Third Year, Semester 2 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            6,
            "PCCST601",
            "Cloud Computing",
            vec![
                (
                    1,
                    "Cloud Fundamentals and Service Models",
                    vec![
                        "Cloud computing characteristics and benefits",
                        "IaaS, PaaS, SaaS service models",
                        "Public, private and hybrid deployment models",
                        "Cloud architecture and reference models",
                    ],
                ),
                (
                    2,
                    "Cloud Infrastructure and Virtualization",
                    vec![
                        "Virtualization concepts and hypervisors",
                        "AWS core services (EC2, S3, RDS)",
                        "Azure and GCP service overview",
                        "Infrastructure as Code (Terraform, CloudFormation)",
                    ],
                ),
                (
                    3,
                    "Containerization and Orchestration",
                    vec![
                        "Docker fundamentals and containers",
                        "Docker images, volumes and networking",
                        "Kubernetes architecture and pods",
                        "Microservices deployment on Kubernetes",
                    ],
                ),
                (
                    4,
                    "Cloud Security, DevOps and Monitoring",
                    vec![
                        "Cloud security challenges and best practices",
                        "Identity and access management (IAM)",
                        "CI/CD pipelines in cloud",
                        "Cloud monitoring and cost optimization",
                    ],
                ),
            ],
        ),
        (
            6,
            "PCCST602",
            "Cyber Security",
            vec![
                (
                    1,
                    "Security Fundamentals and Cryptography",
                    vec![
                        "CIA triad and security principles",
                        "Symmetric and asymmetric cryptography",
                        "Hash functions and digital signatures",
                        "Public key infrastructure (PKI)",
                    ],
                ),
                (
                    2,
                    "Network and Application Security",
                    vec![
                        "Firewalls and intrusion detection systems",
                        "Web application vulnerabilities (OWASP Top 10)",
                        "SQL injection and XSS prevention",
                        "Secure authentication and access control",
                    ],
                ),
                (
                    3,
                    "Malware Analysis and Reverse Engineering",
                    vec![
                        "Types of malware (viruses, worms, ransomware)",
                        "Malware analysis techniques",
                        "Reverse engineering fundamentals",
                        "Memory forensics and analysis",
                    ],
                ),
                (
                    4,
                    "Digital Forensics and Ethical Hacking",
                    vec![
                        "Digital forensic investigation process",
                        "Evidence collection and preservation",
                        "Penetration testing methodology",
                        "Ethical hacking tools and techniques",
                    ],
                ),
            ],
        ),
        (
            6,
            "PCCST603",
            "Data Science",
            vec![
                (
                    1,
                    "Data Science Lifecycle",
                    vec![
                        "CRISP-DM framework",
                        "Data collection and sources",
                        "Data cleaning and preprocessing",
                        "Feature engineering and selection",
                    ],
                ),
                (
                    2,
                    "Statistical Methods and Inference",
                    vec![
                        "Probability distributions and density estimation",
                        "Hypothesis testing and confidence intervals",
                        "Bayesian inference and methods",
                        "Regression analysis (linear and logistic)",
                    ],
                ),
                (
                    3,
                    "Advanced Analytics and Machine Learning",
                    vec![
                        "Ensemble learning (bagging, boosting, stacking)",
                        "Recommendation systems (collaborative filtering)",
                        "Dimensionality reduction techniques",
                        "Time series analysis and forecasting",
                    ],
                ),
                (
                    4,
                    "Ethics, Deployment and Big Data",
                    vec![
                        "AI ethics and fairness",
                        "Model deployment and serving",
                        "ML pipelines and MLOps",
                        "Big data technologies (Spark, Hadoop)",
                    ],
                ),
            ],
        ),
        (
            6,
            "PCCST604",
            "Big Data Analytics",
            vec![
                (
                    1,
                    "Introduction to Big Data Ecosystem",
                    vec![
                        "3Vs of big data (volume, velocity, variety)",
                        "Hadoop ecosystem overview",
                        "HDFS architecture and data replication",
                        "MapReduce programming model",
                    ],
                ),
                (
                    2,
                    "Data Processing with Spark",
                    vec![
                        "Spark architecture and RDDs",
                        "DataFrames and Spark SQL",
                        "Streaming data processing",
                        "Spark MLlib for machine learning",
                    ],
                ),
                (
                    3,
                    "Data Analytics and Mining",
                    vec![
                        "Data mining techniques and algorithms",
                        "Association rule mining (Apriori, FP-Growth)",
                        "Text analytics and natural language processing",
                        "Social network analysis",
                    ],
                ),
                (
                    4,
                    "Data Visualization and Storytelling",
                    vec![
                        "Visualization principles and best practices",
                        "Tableau and dashboard creation",
                        "Interactive visualization with D3.js",
                        "Data-driven storytelling techniques",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S7 — Fourth Year, Semester 1 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            7,
            "PCCST701",
            "Internet of Things",
            vec![
                (
                    1,
                    "IoT Fundamentals and Architecture",
                    vec![
                        "IoT ecosystem and reference architecture",
                        "Sensor technologies and actuators",
                        "Embedded systems basics",
                        "IoT protocols and communication standards",
                    ],
                ),
                (
                    2,
                    "IoT Communication Protocols",
                    vec![
                        "Wireless protocols (BLE, Zigbee, LoRaWAN)",
                        "MQTT and CoAP application protocols",
                        "RESTful IoT and WebSockets",
                        "IPv6 and 6LoWPAN for IoT",
                    ],
                ),
                (
                    3,
                    "IoT Data Management and Edge Computing",
                    vec![
                        "Edge computing and fog computing",
                        "Data analytics at the edge",
                        "IoT data storage and processing",
                        "Cloud integration for IoT",
                    ],
                ),
                (
                    4,
                    "IoT Security and Applications",
                    vec![
                        "IoT security challenges and threats",
                        "Secure boot and firmware updates",
                        "Smart home and smart city applications",
                        "Industrial IoT (IIoT) and Industry 4.0",
                    ],
                ),
            ],
        ),
        (
            7,
            "PCCST702",
            "Blockchain Technologies",
            vec![
                (
                    1,
                    "Blockchain Fundamentals",
                    vec![
                        "Distributed ledger technology",
                        "Cryptographic primitives (hashing, digital signatures)",
                        "Consensus mechanisms (PoW, PoS, DPoS)",
                        "Bitcoin and cryptocurrency concepts",
                    ],
                ),
                (
                    2,
                    "Ethereum and Smart Contracts",
                    vec![
                        "Ethereum platform and EVM",
                        "Smart contract development with Solidity",
                        "Gas, transactions and state management",
                        "DApp development and Web3 integration",
                    ],
                ),
                (
                    3,
                    "Hyperledger and Enterprise Blockchain",
                    vec![
                        "Hyperledger Fabric architecture",
                        "Chaincode development",
                        "Permissioned networks and membership services",
                        "Enterprise blockchain use cases",
                    ],
                ),
                (
                    4,
                    "Blockchain Applications and Future Trends",
                    vec![
                        "Supply chain management on blockchain",
                        "Digital identity and verifiable credentials",
                        "DeFi (Decentralized Finance) and NFTs",
                        "Scalability solutions (sharding, layer 2)",
                    ],
                ),
            ],
        ),
        (
            7,
            "PCCST703",
            "DevOps and CI/CD",
            vec![
                (
                    1,
                    "DevOps Culture and Practices",
                    vec![
                        "DevOps principles and benefits",
                        "Infrastructure as Code concepts",
                        "Configuration management (Ansible, Puppet)",
                        "Version control workflows (Git)",
                    ],
                ),
                (
                    2,
                    "Continuous Integration and Delivery",
                    vec![
                        "CI/CD pipeline design and tools",
                        "Jenkins and GitHub Actions",
                        "Automated testing in CI pipelines",
                        "Artifact management and deployment strategies",
                    ],
                ),
                (
                    3,
                    "Containerization and Orchestration",
                    vec![
                        "Docker compose and multi-container apps",
                        "Kubernetes pods, services and deployments",
                        "Helm charts and package management",
                        "Service mesh and ingress controllers",
                    ],
                ),
                (
                    4,
                    "Monitoring, Logging and SRE",
                    vec![
                        "Monitoring with Prometheus and Grafana",
                        "Centralized logging (ELK stack)",
                        "Site reliability engineering principles",
                        "Incident response and postmortems",
                    ],
                ),
            ],
        ),

        // ═══════════════════════════════════════════════════════════════
        // S8 — Fourth Year, Semester 2 (CSE)
        // ═══════════════════════════════════════════════════════════════
        (
            8,
            "PCCST801",
            "Major Project",
            vec![
                (
                    1,
                    "Project Planning and Literature Survey",
                    vec![
                        "Problem identification and scope definition",
                        "Literature review and gap analysis",
                        "Project proposal and methodology",
                        "Requirement analysis and system design",
                    ],
                ),
                (
                    2,
                    "Implementation and Development",
                    vec![
                        "System architecture and technology stack",
                        "Development and coding standards",
                        "Testing and quality assurance",
                        "Integration and deployment",
                    ],
                ),
                (
                    3,
                    "Evaluation and Results",
                    vec![
                        "Performance analysis and benchmarking",
                        "Results interpretation and discussion",
                        "Comparison with existing solutions",
                        "Validation and verification",
                    ],
                ),
                (
                    4,
                    "Documentation and Presentation",
                    vec![
                        "Technical report writing",
                        "Research paper preparation",
                        "Presentation and demonstration skills",
                        "Project poster and portfolio",
                    ],
                ),
            ],
        ),
    ];

    // ── 5. Insert all subjects, modules, topics & topic_stats ──────────
    for (sem_index, code, name, modules) in &subjects_data {
        let sem_id = semester_ids[*sem_index - 1];

        let subject_id = sqlx::query_scalar::<_, uuid::Uuid>(
            "INSERT INTO subjects (id, scheme_id, branch_id, semester_id, subject_code, subject_name, active)
             VALUES (uuid_generate_v4(), $1, $2, $3, $4, $5, true)
             RETURNING id",
        )
        .bind(scheme_id)
        .bind(cse_id)
        .bind(sem_id)
        .bind(code)
        .bind(name)
        .fetch_one(pool)
        .await?;

        for (mod_idx, mod_name, topics) in modules {
            let module_id = sqlx::query_scalar::<_, uuid::Uuid>(
                "INSERT INTO modules (id, subject_id, module_index, module_name)
                 VALUES (uuid_generate_v4(), $1, $2, $3)
                 RETURNING id",
            )
            .bind(subject_id)
            .bind(mod_idx)
            .bind(mod_name)
            .fetch_one(pool)
            .await?;

            for topic_name in topics {
                let topic_id = sqlx::query_scalar::<_, uuid::Uuid>(
                    "INSERT INTO topics (id, module_id, topic_name, normalized_name, active)
                     VALUES (uuid_generate_v4(), $1, $2, $2, true)
                     RETURNING id",
                )
                .bind(module_id)
                .bind(topic_name)
                .fetch_one(pool)
                .await?;

                // Initialize topic_stats for each topic
                sqlx::query(
                    "INSERT INTO topic_stats (id, topic_id)
                     VALUES (uuid_generate_v4(), $1)",
                )
                .bind(topic_id)
                .execute(pool)
                .await?;
            }
        }
    }

    tracing::info!("Database seeded successfully with KTU 2024 Scheme CSE data ({} subjects)", subjects_data.len());
    Ok(())
}

// ── Question Paper Seeding ────────────────────────────────────────────────

/// Loaded subject topic tree from the database
#[derive(Clone, Debug)]
struct QSubject {
    id: Uuid,
    #[allow(dead_code)]
    code: String,
    semester_number: i32,
    modules: Vec<QModule>,
}

#[derive(Clone, Debug)]
struct QModule {
    #[allow(dead_code)]
    id: Uuid,
    name: String,
    #[allow(dead_code)]
    index: i32,
    topics: Vec<QTopic>,
}

#[derive(Clone, Debug)]
struct QTopic {
    id: Uuid,
    name: String,
}

/// Seed realistic question paper data so the analysis engine produces meaningful priority scores.
pub async fn seed_question_papers(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Check if question papers already exist
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM question_papers")
        .fetch_one(pool)
        .await?;
    if count.0 > 0 {
        tracing::info!("Question papers already exist, skipping");
        return Ok(());
    }

    tracing::info!("Seeding question paper data...");

    // ── 1. Load all subjects with modules & topics ────────────────────
    let subjects = load_subjects_with_topics(pool).await?;

    let mut total_papers = 0;

    for subject in &subjects {
        // Determine number of question papers based on semester
        let num_papers = match subject.semester_number {
            1 | 2 => 3, // First year: most papers
            3 | 4 => 2, // Second year
            5..=7 => 1, // Third/Fourth year
            _ => 0, // S8 = Major Project, no exam papers
        };

        if num_papers == 0 {
            continue;
        }

        total_papers += num_papers;

        // Create papers for this subject
        for paper_idx in 0..num_papers {
            let year = [2024, 2025, 2026][paper_idx % 3];
            let term = match paper_idx {
                0 => "December",
                1 => "June",
                _ => "December",
            };

            // Create the question paper record
            let paper_id = sqlx::query_scalar::<_, Uuid>(
                "INSERT INTO question_papers (id, subject_id, exam_year, exam_term, source_type)
                 VALUES (uuid_generate_v4(), $1, $2, $3, 'manual')
                 RETURNING id",
            )
            .bind(subject.id)
            .bind(year)
            .bind(term)
            .fetch_one(pool)
            .await?;

            // Generate and insert 10 questions for this paper
            let questions = generate_questions_for_paper(subject, paper_idx, num_papers);

            for q in &questions {
                let question_id = sqlx::query_scalar::<_, Uuid>(
                    "INSERT INTO questions (id, question_paper_id, question_text, marks, order_index)
                     VALUES (uuid_generate_v4(), $1, $2, $3, $4)
                     RETURNING id",
                )
                .bind(paper_id)
                .bind(&q.text)
                .bind(q.marks)
                .bind(q.order_index)
                .fetch_one(pool)
                .await?;

                // Map question to its topic(s)
                for topic_id in &q.topic_ids {
                    sqlx::query(
                        "INSERT INTO question_topic_map (id, question_id, topic_id, confidence)
                         VALUES (uuid_generate_v4(), $1, $2, 0.95)",
                    )
                    .bind(question_id)
                    .bind(topic_id)
                    .execute(pool)
                    .await?;
                }
            }
        }
    }

    // ── 3. Update topic_stats for all topics ─────────────────────────
    update_all_topic_stats(pool).await?;

    tracing::info!("Question paper seeding complete: {} papers across {} subjects", total_papers, subjects.len());
    Ok(())
}

/// Load all subjects with their modules and topics from the database
async fn load_subjects_with_topics(pool: &PgPool) -> Result<Vec<QSubject>, sqlx::Error> {
    // Get all active subjects with their semester info
    let subjects_raw = sqlx::query_as::<_, (Uuid, String, i32)>(
        r#"SELECT s.id, s.subject_code, sem.semester_number
           FROM subjects s
           JOIN semesters sem ON sem.id = s.semester_id
           WHERE s.active = true
           ORDER BY sem.semester_number, s.subject_code"#,
    )
    .fetch_all(pool)
    .await?;

    let mut subjects = Vec::new();

    for (subject_id, code, sem_num) in subjects_raw {
        // Get modules for this subject
        let modules_raw = sqlx::query_as::<_, (Uuid, String, i32)>(
            "SELECT id, module_name, module_index FROM modules WHERE subject_id = $1 ORDER BY module_index",
        )
        .bind(subject_id)
        .fetch_all(pool)
        .await?;

        let mut modules = Vec::new();

        for (module_id, mod_name, mod_idx) in modules_raw {
            // Get topics for this module
            let topics_raw = sqlx::query_as::<_, (Uuid, String)>(
                "SELECT id, topic_name FROM topics WHERE module_id = $1 AND active = true ORDER BY id",
            )
            .bind(module_id)
            .fetch_all(pool)
            .await?;

            let topics: Vec<QTopic> = topics_raw
                .into_iter()
                .map(|(id, name)| QTopic { id, name })
                .collect();

            modules.push(QModule {
                id: module_id,
                name: mod_name,
                index: mod_idx,
                topics,
            });
        }

        subjects.push(QSubject {
            id: subject_id,
            code,
            semester_number: sem_num,
            modules,
        });
    }

    Ok(subjects)
}

/// A generated question with its associated topic IDs
struct GeneratedQuestion {
    text: String,
    marks: i32,
    order_index: i32,
    topic_ids: Vec<Uuid>,
}

/// Generate 10 questions for a subject's question paper.
/// Uses the topic's position within its module to determine frequency:
///   position 0 (first topic) → high priority — appears in most papers
///   position 1 → medium priority — appears in some papers
///   position 2 → low priority — appears rarely
///   position 3 → rare — appears only in module-level questions
fn generate_questions_for_paper(
    subject: &QSubject,
    paper_idx: usize,
    _num_papers: usize,
) -> Vec<GeneratedQuestion> {
    // Build ID-indexed references for each position bucket
    // We store clones of UUIDs and borrow topic names as strings
    let mut pos0_ids: Vec<Uuid> = Vec::new(); // high priority — first topic in each module
    let mut pos0_names: Vec<String> = Vec::new();
    let mut pos1_ids: Vec<Uuid> = Vec::new(); // medium priority
    let mut pos1_names: Vec<String> = Vec::new();
    let mut pos2_ids: Vec<Uuid> = Vec::new(); // low priority
    let mut pos2_names: Vec<String> = Vec::new();
    let mut pos3_ids: Vec<Uuid> = Vec::new(); // rare
    let mut pos3_names: Vec<String> = Vec::new();

    for m in &subject.modules {
        for (i, t) in m.topics.iter().enumerate() {
            match i {
                0 => {
                    pos0_ids.push(t.id);
                    pos0_names.push(t.name.clone());
                }
                1 => {
                    pos1_ids.push(t.id);
                    pos1_names.push(t.name.clone());
                }
                2 => {
                    pos2_ids.push(t.id);
                    pos2_names.push(t.name.clone());
                }
                _ => {
                    pos3_ids.push(t.id);
                    pos3_names.push(t.name.clone());
                }
            }
        }
    }

    // Helper to get a topic's ID and name from buckets with rotation
    let get_topic = |ids: &[Uuid], names: &[String], rot: usize, offset: usize| -> Option<(Uuid, String)> {
        if ids.is_empty() {
            return None;
        }
        let idx = (rot + offset) % ids.len();
        Some((ids[idx], names[idx].clone()))
    };

    let hc = pos0_ids.len().max(1);
    let mc = pos1_ids.len().max(1);
    let lc = pos2_ids.len().max(1);
    let rc = pos3_ids.len().max(1);

    let high_rot = paper_idx % hc;
    let med_rot = (paper_idx + 1) % mc;
    let low_rot = (paper_idx + 2) % lc;
    let rare_rot = (paper_idx + 3) % rc;

    let mut questions = Vec::new();
    let mut order = 1;

    // ── Q1: 15 marks — Module-level comprehensive ──
    if let Some(mod1) = subject.modules.get(paper_idx % 4) {
        if let (Some(t1), Some(t2)) = (mod1.topics.first(), mod1.topics.get(1)) {
            questions.push(GeneratedQuestion {
                text: format!(
                    "Explain the fundamental concepts of {} covering '{}' and '{}' with detailed examples and applications.",
                    mod1.name, t1.name, t2.name
                ),
                marks: 15,
                order_index: order,
                topic_ids: vec![t1.id, t2.id],
            });
            order += 1;
        }
    }

    // ── Q2: 15 marks — Second module-level ──
    if let Some(mod2) = subject.modules.get((paper_idx + 2) % 4) {
        if let (Some(t1), Some(t2)) = (mod2.topics.first(), mod2.topics.get(1)) {
            questions.push(GeneratedQuestion {
                text: format!(
                    "Discuss the key principles of {} with emphasis on '{}' and '{}'. Provide suitable examples.",
                    mod2.name, t1.name, t2.name
                ),
                marks: 15,
                order_index: order,
                topic_ids: vec![t1.id, t2.id],
            });
            order += 1;
        }
    }

    // ── Q3: 10 marks — High priority topic ──
    if let Some((tid, tname)) = get_topic(&pos0_ids, &pos0_names, high_rot, 0) {
        questions.push(GeneratedQuestion {
            text: format!("Describe in detail the concept of '{}' with suitable examples, diagrams, and real-world applications.", tname),
            marks: 10,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q4: 10 marks — Second high priority ──
    if let Some((tid, tname)) = get_topic(&pos0_ids, &pos0_names, high_rot, 1) {
        questions.push(GeneratedQuestion {
            text: format!("Explain '{}' in detail. Discuss its significance and applications in the relevant field.", tname),
            marks: 10,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q5: 5 marks — Medium priority ──
    if let Some((tid, tname)) = get_topic(&pos1_ids, &pos1_names, med_rot, 0) {
        questions.push(GeneratedQuestion {
            text: format!("Explain the concept of '{}' with relevant examples.", tname),
            marks: 5,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q6: 5 marks — Second medium priority ──
    if let Some((tid, tname)) = get_topic(&pos1_ids, &pos1_names, med_rot, 1) {
        questions.push(GeneratedQuestion {
            text: format!("Discuss the importance and applications of '{}'.", tname),
            marks: 5,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q7: 5 marks — Low priority ──
    if let Some((tid, tname)) = get_topic(&pos2_ids, &pos2_names, low_rot, 0) {
        questions.push(GeneratedQuestion {
            text: format!("Write a detailed note on '{}'.", tname),
            marks: 5,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q8: 4 marks — Low priority ──
    if let Some((tid, tname)) = get_topic(&pos2_ids, &pos2_names, low_rot, 1) {
        questions.push(GeneratedQuestion {
            text: format!("Explain briefly the concept of '{}' highlighting its key features.", tname),
            marks: 4,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q9: 3 marks — Rare topic ──
    if let Some((tid, tname)) = get_topic(&pos3_ids, &pos3_names, rare_rot, 0) {
        questions.push(GeneratedQuestion {
            text: format!("Define '{}' and list its key characteristics.", tname),
            marks: 3,
            order_index: order,
            topic_ids: vec![tid],
        });
        order += 1;
    }

    // ── Q10: 3 marks — Second rare topic ──
    if let Some((tid, tname)) = get_topic(&pos3_ids, &pos3_names, rare_rot, 1) {
        questions.push(GeneratedQuestion {
            text: format!("Write short notes on '{}'.", tname),
            marks: 3,
            order_index: order,
            topic_ids: vec![tid],
        });
    }

    questions
}

/// Raw topic stat row from the database
#[derive(Debug, Clone)]
struct TopicStatRow {
    topic_id: Uuid,
    subject_id: Uuid,
    frequency_count: i32,
    total_marks_count: i32,
    avg_marks: f64,
    last_seen_year: Option<i32>,
}

/// Update all topic_stats including computed recency_score and priority_score.
///
/// Scores are computed per-subject (relative to other topics in the same subject)
/// using the same formula as the analysis engine:
///   - recency_score: based on how recently the topic appeared (0-1)
///   - priority_score: weighted combination of frequency (25%), marks (30%),
///     recency (25%), with small bonuses for absolute frequency and marks
async fn update_all_topic_stats(pool: &PgPool) -> Result<(), sqlx::Error> {
    // Load all topics with their subject context and aggregate question stats
    let rows = sqlx::query_as::<_, (Uuid, Uuid, i64, i64, Option<f64>, Option<i32>)>(
        r#"SELECT
            t.id,
            m.subject_id,
            COALESCE(COUNT(DISTINCT q.id), 0)::bigint as freq,
            COALESCE(SUM(q.marks), 0)::bigint as total_marks,
            AVG(q.marks::float) as avg_m,
            MAX(qp.exam_year) as last_year
           FROM topics t
           JOIN modules m ON t.module_id = m.id
           LEFT JOIN question_topic_map qtm ON qtm.topic_id = t.id
           LEFT JOIN questions q ON qtm.question_id = q.id
           LEFT JOIN question_papers qp ON q.question_paper_id = qp.id
           WHERE t.active = true
           GROUP BY t.id, m.subject_id
           ORDER BY m.subject_id"#,
    )
    .fetch_all(pool)
    .await?;

    let all_topics: Vec<TopicStatRow> = rows
        .into_iter()
        .map(|(tid, sid, freq, marks, avg, last_yr)| TopicStatRow {
            topic_id: tid,
            subject_id: sid,
            frequency_count: freq as i32,
            total_marks_count: marks as i32,
            avg_marks: avg.unwrap_or(0.0),
            last_seen_year: last_yr,
        })
        .collect();

    // Group topics by subject to compute per-subject max values
    let mut by_subject: std::collections::HashMap<Uuid, Vec<&TopicStatRow>> =
        std::collections::HashMap::new();
    for topic in &all_topics {
        by_subject
            .entry(topic.subject_id)
            .or_default()
            .push(topic);
    }

    let current_year = 2026;
    let mut updated_count = 0;

    for topics in by_subject.values() {
        // Compute max values for this subject (for normalization)
        let max_freq = topics
            .iter()
            .map(|t| t.frequency_count)
            .max()
            .unwrap_or(1)
            .max(1);
        let max_marks = topics
            .iter()
            .map(|t| t.total_marks_count)
            .max()
            .unwrap_or(1)
            .max(1);

        for topic in topics {
            // 1. Frequency score (normalized 0-1)
            let freq_score = topic.frequency_count as f64 / max_freq as f64;

            // 2. Marks score (normalized 0-1, weighted toward total)
            let marks_score = (topic.total_marks_count as f64 / max_marks as f64) * 0.6
                + (topic.avg_marks / 10.0).min(1.0) * 0.4;

            // 3. Recency score
            let recency_score = match topic.last_seen_year {
                Some(year) => {
                    let years_ago = (current_year - year).max(0) as f64;
                    (1.0 - (years_ago / 5.0)).max(0.0)
                }
                None => 0.3,
            };

            // 4. Priority score (baseline, no time pressure — time_pressure = 1.0)
            // Same formula as analysis_engine.rs but without time_pressure modifier
            // (days_remaining is not available at seed time)
            let priority_score = (freq_score * 0.25
                + marks_score * 0.30
                + recency_score * 0.25)
                + (topic.frequency_count as f64 * 0.05)
                + (topic.total_marks_count as f64 * 0.05 / max_marks as f64);

            let priority_rounded = (priority_score * 100.0).round() / 100.0;
            let recency_rounded = (recency_score * 100.0).round() / 100.0;

            sqlx::query(
                "UPDATE topic_stats SET
                 frequency_count = $1, total_marks_count = $2,
                 avg_marks = $3, last_seen_year = $4,
                 recency_score = $5, priority_score = $6,
                 updated_at = NOW()
                 WHERE topic_id = $7",
            )
            .bind(topic.frequency_count)
            .bind(topic.total_marks_count)
            .bind(topic.avg_marks)
            .bind(topic.last_seen_year)
            .bind(recency_rounded)
            .bind(priority_rounded)
            .bind(topic.topic_id)
            .execute(pool)
            .await?;

            updated_count += 1;
        }
    }

    tracing::info!(
        "Topic stats updated for {} topics across {} subjects",
        updated_count,
        by_subject.len()
    );
    Ok(())
}
