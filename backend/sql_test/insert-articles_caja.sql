USE [WTPV_CALEIA_TEST];
GO
SET IDENTITY_INSERT [dbo].[TP_ARTICULO_CAJA] ON
GO
INSERT [dbo].[TP_ARTICULO_CAJA] ([Cajtpv], [Articulo], [Coste], [Fecha_Modificado], [Usuario], [Codigoid])
VALUES
(N'0001', CAST(1 AS Numeric(9, 0)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:28:44.303' AS DateTime), N'MASTER', CAST(40895 AS Numeric(18, 0))),
(N'0002', CAST(1 AS Numeric(9, 0)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:28:44.303' AS DateTime), N'MASTER', CAST(40896 AS Numeric(18, 0))),
(N'0003', CAST(1 AS Numeric(9, 0)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:28:44.303' AS DateTime), N'MASTER', CAST(40896 AS Numeric(18, 0))),
(N'0004', CAST(1 AS Numeric(9, 0)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:28:44.303' AS DateTime), N'MASTER', CAST(40896 AS Numeric(18, 0)))
GO
SET IDENTITY_INSERT [dbo].[TP_ARTICULO_CAJA] OFF
GO
