import numpy

arr_n = [3000, 5000, 10000, 15000, 30000, 60000]
arr_t = [2, 5, 22, 54, 196, 839]

k = len(arr_n)
sum_n = sum(i for i in arr_n)
sum_n2 = sum(i ** 2 for i in arr_n)
sum_n3 = sum(i ** 3 for i in arr_n)
sum_n4 = sum(i ** 4 for i in arr_n)
sum_t = sum(i for i in arr_t)

sum_tn = 0
for i in range(0, k):
    sum_tn += arr_n[i] * arr_t[i]

sum_tn2 = 0
for i in range(0, k):
    sum_tn2 += arr_n[i] ** 2 * arr_t[i]

system_1 = numpy.array([
    [sum_n4, sum_n3, sum_n2],
    [sum_n3, sum_n2, sum_n],
    [sum_n2, sum_n, k]
])
t_1 = numpy.array([sum_tn2, sum_tn, sum_t])

a_b_c = numpy.linalg.solve(system_1, t_1)
print("n^2")
print(a_b_c)
print("\n")


arr_n = [50000, 100000, 150000, 200000, 250000, 300000]
arr_t = [4, 11, 22, 30, 42, 57]

k = len(arr_n)
sum_n = sum(i for i in arr_n)
sum_n2 = sum(i ** 2 for i in arr_n)
sum_t = sum(i for i in arr_t)

sum_tn = 0
for i in range(0, k):
    sum_tn += arr_n[i] * arr_t[i]

sum_tnlogn = 0
for i in range(0, k):
    sum_tnlogn += arr_n[i] * arr_t[i] * numpy.log2(arr_n[i])

sum_nlogn = 0
for i in range(0, k):
    sum_nlogn += arr_n[i] * numpy.log2(arr_n[i])

sum_n2logn = 0
for i in range(0, k):
    sum_n2logn += arr_n[i] ** 2 * numpy.log2(arr_n[i])

sum_n2logn2 = 0
for i in range(0, k):
    sum_n2logn2 += arr_n[i] ** 2 * numpy.log2(arr_n[i]) ** 2

system_2 = numpy.array([
    [sum_n2logn2, sum_n2logn, sum_nlogn],
    [sum_n2logn, sum_n2, sum_n],
    [sum_nlogn, sum_n, k]
])
t_2 = numpy.array([sum_tnlogn, sum_tn, sum_t])

a_b_c = numpy.linalg.solve(system_2, t_2)
print("nlogn")
print(a_b_c)
