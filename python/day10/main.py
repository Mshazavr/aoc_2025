import numpy as np
import cvxpy as cp

TEST_PATHS = [
    "tests/test0.txt",
    "tests/test1.txt",
]


if __name__ == "__main__":
    test_path = TEST_PATHS[1]
    with open(test_path, "r") as f:
        test_contents = f.read()
    
    machine_texts = test_contents.split("\n");
    
    total_cost = 0
    
    for machine_text in machine_texts:
        components = machine_text.split()
        N = len(components[0]) - 2
        M = len(components) - 2
        
        matrix_columns = []
        for button in components[1:-1]:
            indices = [int(part.strip()) for part in button[1:-1].split(",")]
            matrix_column = [0.0 for _ in range(N)]
            for index in indices:
                matrix_column[index] = 1
            matrix_columns.append(matrix_column)
        
        b_vector = [int(part.strip()) for part in components[-1][1:-1].split(",")]
        
        A = np.array(matrix_columns).T 
        B = np.array(b_vector)
        
        x = cp.Variable(M, integer=True)
        objective = cp.Minimize(cp.norm1(x))
        constraints = [
            A @ x == B,
            x >= 0
        ]

        problem = cp.Problem(objective, constraints)
        problem.solve()

        print("x:", [float(round(x_i, 2)) for x_i in x.value])
        total_cost += int(problem.value)
    
    print(total_cost)