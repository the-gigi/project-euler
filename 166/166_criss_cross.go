/*


A 4x4 grid is filled with digits d, 0 ≤ d ≤ 9.

It can be seen that in the grid

6 3 3 0
5 0 4 3
0 7 1 4
1 2 4 5

the sum of each row and each column has the value 12. Moreover the sum of each diagonal is also 12.

In how many ways can you fill a 4x4 grid with the digits d, 0 ≤ d ≤ 9 so that each row, each column, and both diagonals have the same sum?

 */

package main

import (
	"fmt"
	"time"
)


func main() {
	t := time.Now()
	counter := 0
	for i1 := 0; i1 < 10; i1++ {
		for i2 := 0; i2 < 10; i2++ {
			for i3 := 0; i3 < 10; i3++ {
				for i4 := 0; i4 < 10; i4++ {
					row1 := i1 + i2 + i3 + i4
					for i5 := 0; i5 < 10; i5++ {
						for i6 := 0; i6 < 10; i6++ {
							for i7 := 0; i7 < 10; i7++ {
								for i8 := 0; i8 < 10; i8++ {
									row2 := i5 + i6 + i7 + i8
									if row2 != row1 {
										continue
									}
									for i9 := 0; i9 < 10; i9++ {
										for i10 := 0; i10 < 10; i10++ {
											for i11 := 0; i11 < 10; i11++ {
												for i12 := 0; i12 < 10; i12++ {
													row3 := i9 + i10 + i11 + i12
													if row3 != row1 {
														continue
													}
													for i13 := 0; i13 < 10; i13++ {
														col1 := i1 + i5 + i9 + i13
														if col1 != row1 {
															continue
														}
														diag2 := i4 + i7 + i10 + i13
														if diag2 != row1 {
															continue
														}
														for i14 := 0; i14 < 10; i14++ {
															col2 := i2 + i6 + i10 + i14
															if col2 != row1 {
																continue
															}
															for i15 := 0; i15 < 10; i15++ {
																col3 := i3 + i7 + i11 + i15
																if col3 != row1 {
																	continue
																}
																for i16 := 0; i16 < 10; i16++ {
																	row4 := i13 + i14 + i15 + i16
																	if row4 != row1 {
																		continue
																	}
																	col4 := i4 + i8 + i12 + i16
																	if col4 != row1 {
																		continue
																	}
																	diag1 := i1 + i6 + i11 + i16
																	if diag1 != row1 {
																		continue
																	}
																	counter +=1
																}
															}
														}
													}
												}
											}
										}
									}
								}
							}
						}
					}
				}
			}
		}
	}

	duration := time.Now().Unix() - t.Unix()
	fmt.Println("criss cross answer:", counter, duration)
}
