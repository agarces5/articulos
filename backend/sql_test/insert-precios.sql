USE [WTPV_CALEIA_TEST];
GO
SET IDENTITY_INSERT [dbo].[TP_PRECIOS] ON
GO
INSERT INTO [dbo].[TP_PRECIOS]
([Cajtpv], [Tipotarifa], [Articulo], [Precio], [Porcentaje_Combinado], [Precio_Combinado], [Fecha_Modificado], [Usuario], [Codigoid])
VALUES
(N'0001', N'GENE', CAST(1 AS Numeric(9, 0)), CAST(3.3000 AS Numeric(18, 4)), CAST(100.00 AS Numeric(7, 2)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:31:27.363' AS DateTime), N'AS400', CAST(147821 AS Numeric(18, 0))),
(N'0001', N'HAHO', CAST(1 AS Numeric(9, 0)), CAST(3.3000 AS Numeric(18, 4)), CAST(100.00 AS Numeric(7, 2)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:31:27.363' AS DateTime), N'AS400', CAST(147818 AS Numeric(18, 0))),
(N'0001', N'MYP+', CAST(1 AS Numeric(9, 0)), CAST(3.3000 AS Numeric(18, 4)), CAST(100.00 AS Numeric(7, 2)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:31:27.363' AS DateTime), N'AS400', CAST(147822 AS Numeric(18, 0))),
(N'0001', N'REFI', CAST(1 AS Numeric(9, 0)), CAST(3.3000 AS Numeric(18, 4)), CAST(100.00 AS Numeric(7, 2)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:31:27.363' AS DateTime), N'AS400', CAST(147819 AS Numeric(18, 0))),
(N'0001', N'TIN+', CAST(1 AS Numeric(9, 0)), CAST(3.3000 AS Numeric(18, 4)), CAST(100.00 AS Numeric(7, 2)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:31:27.363' AS DateTime), N'AS400', CAST(147817 AS Numeric(18, 0))),
(N'0001', N'TINC', CAST(1 AS Numeric(9, 0)), CAST(3.3000 AS Numeric(18, 4)), CAST(100.00 AS Numeric(7, 2)), CAST(0.0000 AS Numeric(18, 4)), CAST(N'2021-08-31T18:31:27.363' AS DateTime), N'AS400', CAST(147820 AS Numeric(18, 0)))
GO

SET IDENTITY_INSERT [dbo].[TP_PRECIOS] OFF
GO
